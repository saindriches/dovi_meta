use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{bail, ensure, Result};
use dolby_vision::rpu::utils::parse_rpu_file;
use itertools::{Itertools, MinMaxResult};
use quick_xml::events::Event;
use quick_xml::se::Serializer;
use quick_xml::{Reader, Writer};
use serde::Serialize;

use crate::cmv40::{Characteristics, EditRate, Output, Shot, Track};
use crate::commands::convert::ConvertArgs;
use crate::metadata::levels::Level11;
use crate::metadata::levels::Level5;
use crate::MDFType::{CMV29, CMV40};
use crate::{
    cmv40, display, IntoCMV29, Level254, Level6, UHD_AR, UHD_HEIGHT, UHD_WIDTH, XML_PREFIX,
};

#[derive(Debug, Default)]
pub struct Converter {
    frame_index: usize,
    // scene_count: usize,
    invalid_frame_count: usize,
    first_valid_frame_index: Option<usize>,
    shots: Option<Vec<Shot>>,
    last_shot: Shot,
    track: Track,
    level5: Option<Level5>,
    level11: Option<Level11>,
    level254: Option<Level254>,
}

impl Converter {
    pub fn convert(args: ConvertArgs) -> Result<()> {
        let input = match args.input {
            Some(input) => input,
            None => bail!("No input file provided."),
        };

        ensure!(args.count != Some(0), "Invalid specified frame count.");

        let canvas = Converter::parse_canvas_ar(args.size)?;

        println!("Parsing RPU file...");

        let rpus = parse_rpu_file(input)?;

        let mut count = if let Some(count) = args.count {
            if count + args.skip > rpus.len() {
                println!("Specified frame count exceeds the end.");
                rpus.len()
            } else {
                count
            }
        } else {
            rpus.len()
        };

        let mut converter = Converter::default();

        let edit_rate = EditRate::from(args.rate);
        edit_rate.validate()?;

        println!("Converting RPU file...");

        let mut targets_map = HashMap::new();

        // Parse shot-based and frame-based metadata
        for rpu in rpus {
            if count > 0 {
                if let Some(ref vdr) = rpu.vdr_dm_data {
                    if converter.frame_index >= args.skip {
                        let frame_index = converter.frame_index - args.skip;
                        // TODO: Use real offset if first valid frame index is not 0?

                        if converter.first_valid_frame_index.is_none()
                            || vdr.scene_refresh_flag == 1
                        {
                            match &mut converter.shots {
                                // Initialize
                                None => {
                                    converter.shots = Some(Vec::new());
                                }
                                Some(shots) => {
                                    shots.push(converter.last_shot.clone());
                                }
                            }

                            converter.last_shot = Shot::with_canvas(vdr, canvas);
                            converter.last_shot.update_record(Some(frame_index), None);

                            // FIXME: Assume input rpu file is valid,
                            // so only use the first valid frame to get global information we need
                            if converter.first_valid_frame_index.is_none() {
                                if converter.invalid_frame_count > 0 {
                                    println!(
                                        "Skipped {} invalid frame(s) from start.",
                                        converter.invalid_frame_count
                                    );
                                    converter.invalid_frame_count = 0;
                                }
                                if args.keep_offset {
                                    converter.last_shot.update_record(None, Some(args.skip));
                                    converter.frame_index += args.skip;
                                }
                                converter.first_valid_frame_index = Some(frame_index);
                                converter.track = Track::with_single_vdr(vdr);

                                if !args.use_level6 {
                                    converter.track.level6 = Some(Level6::default());
                                }

                                converter.level254 = converter.track.plugin_node.level254.clone();

                                converter.track.edit_rate = if converter.level254.is_none() {
                                    CMV29(edit_rate)
                                } else {
                                    CMV40(edit_rate)
                                };
                            };
                        } else {
                            converter.last_shot.update_record(None, None);
                            if !args.drop_per_frame {
                                converter
                                    .last_shot
                                    .append_metadata(&Shot::with_canvas(vdr, canvas));
                            }
                        }

                        if let Some(d) = display::Characteristics::get_targets(vdr) {
                            d.iter().for_each(|c| {
                                let target = Characteristics::from(c.clone());
                                targets_map.entry(target.id).or_insert(target);
                            })
                        }

                        count -= 1;
                    }

                    converter.frame_index += 1;
                } else {
                    // Should not happen
                    if converter.first_valid_frame_index.is_some() {
                        // Invalid RPU in the middle of sequence, use last valid frame
                        converter.frame_index += 1;
                        converter.last_shot.update_record(None, None);
                        if let Some(ref mut frames) = converter.last_shot.frames {
                            if let Some(frame) = frames.pop() {
                                frames.push(frame.clone());
                                frames.push(frame);
                            }
                        }

                        count -= 1;
                    }

                    converter.invalid_frame_count += 1;
                }
            }
        }

        if converter.invalid_frame_count > 0 {
            println!(
                "Skipped {} invalid frame(s) in the middle, replaced with previous metadata.",
                converter.invalid_frame_count
            );
        }

        // Push remained shot
        if converter.shots.is_none() {
            converter.shots = Some(Vec::new());
        }

        if let Some(ref mut shots) = converter.shots {
            shots.push(converter.last_shot.clone());

            let mut targets = targets_map.values().cloned().collect::<Vec<_>>();
            if !targets.is_empty() {
                targets.sort_by_key(|c| c.id);
                converter.track.plugin_node.dv_global_data.target_displays = Some(targets);
            }

            // FIXME: THE IMPLEMENTATION FOR LEVEL5 IS WRONG !!!
            let mut level5_map = HashMap::new();
            let mut level11_map = HashMap::new();

            shots.iter().for_each(|shot| {
                *level5_map
                    .entry(&shot.plugin_node.dv_dynamic_data.level5)
                    .or_insert(0) += 1_usize;

                *level11_map.entry(&shot.plugin_node.level11).or_insert(0) += 1_usize;
            });

            converter.level5 = Some(Self::get_global_ar(level5_map, canvas));

            // Choose the most common level11 as track-level metadata,
            converter.level11 = Self::get_common(level11_map);

            // and remove them in shot-level.
            shots.iter_mut().for_each(|shot| {
                if shot.plugin_node.dv_dynamic_data.level5 == converter.level5 {
                    shot.plugin_node.dv_dynamic_data.level5 = None;
                };

                if let Some(ref mut frames) = shot.frames {
                    frames.iter_mut().for_each(|frame| {
                        if frame.dv_dynamic_data.level5 == converter.level5 {
                            frame.dv_dynamic_data.level5 = None;
                        }
                    })
                }

                if shot.plugin_node.level11 == converter.level11 {
                    shot.plugin_node.level11 = None;
                };
            });
        }

        converter.track.shots = converter.shots;
        converter.track.plugin_node.level11 = converter.level11;

        let output = Output::with_level5(converter.track, converter.level5);

        let md = cmv40::DolbyLabsMDF::with_single_output(output)?;

        let mut serializer_buffer = Vec::new();
        let writer = Writer::new(&mut serializer_buffer);
        let mut ser = Serializer::new(writer.into_inner());

        if converter.level254.is_none() {
            println!("CM v2.9 RPU found, saving as v2.0.5 XML...");
            md.into_cmv29().serialize(&mut ser)?;
        } else {
            println!("CM v4.0 RPU found, saving as v{} XML...", md.version);
            md.serialize(&mut ser)?;
        }

        let output = if let Some(output) = args.output {
            output
        } else {
            println!("No output file provided, writing to metadata.xml at current path...");
            "./metadata.xml".into()
        };

        let mut output_buffer = BufWriter::new(File::create(output)?);
        write!(
            output_buffer,
            "{}{}",
            XML_PREFIX,
            Self::prettify_xml(String::from_utf8(serializer_buffer)?)
        )?;

        Ok(())
    }

    /// None: Standard UHD
    fn parse_canvas_ar(vec: Vec<usize>) -> Result<Option<(usize, usize)>> {
        ensure!(
            vec.len() == 2,
            "Invalid canvas size. Use 'x' as delimiter, like 3840x2160"
        );
        ensure!(vec[0] != 0 && vec[1] != 0, "Invalid canvas size.");
        let canvas = (vec[0], vec[1]);
        let canvas = if canvas == (UHD_WIDTH, UHD_HEIGHT) {
            None
        } else {
            Some(canvas)
        };

        // stdout().flush().ok();

        Ok(canvas)
    }

    fn get_global_ar(
        map: HashMap<&Option<Level5>, usize>,
        canvas: Option<(usize, usize)>,
    ) -> Level5 {
        let canvas_ar = match canvas {
            Some((width, height)) => width as f32 / height as f32,
            None => UHD_AR,
        };

        let minmax = map
            .into_iter()
            .filter(|(value, _)| value.is_some())
            .map(|(value, _)| value.clone().unwrap())
            .minmax();

        match minmax {
            MinMaxResult::NoElements => Level5::from(canvas_ar),
            MinMaxResult::OneElement(b) => b,
            MinMaxResult::MinMax(b_min, b_max) => {
                let b_canvas = Level5::from(canvas_ar);
                // if b_min > b_canvas {
                //     // all letterbox types are up/bottom
                //     b_min
                // } else if b_max < b_canvas {
                //     // all letterbox types are left/right
                //     b_max
                // } else {
                //     // Mixed type, or no letterbox
                //     b_canvas
                // }
                b_canvas.clamp(b_min, b_max)
            }
        }
    }

    fn get_common<K, V>(map: HashMap<&Option<K>, V>) -> Option<K>
    where
        K: Clone,
        V: Copy + Ord,
    {
        map.into_iter()
            .filter(|(value, _)| value.is_some())
            .max_by_key(|&(_, count)| count)
            .and_then(|(value, _)| value.clone())
    }

    // https://gist.github.com/lwilli/14fb3178bd9adac3a64edfbc11f42e0d/forks
    fn prettify_xml(xml: String) -> String {
        let mut buf = Vec::new();

        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

        loop {
            let ev = reader.read_event_into(&mut buf);

            match ev {
                Ok(Event::Eof) => break,
                Ok(event) => writer.write_event(event),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            }
            .expect("Failed to parse XML");

            buf.clear();
        }

        let result = std::str::from_utf8(&writer.into_inner())
            .expect("Failed to convert a slice of bytes to a string slice")
            .to_string();

        result
    }
}
