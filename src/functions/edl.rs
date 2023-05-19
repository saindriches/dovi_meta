use std::ffi::OsStr;
use std::fs::File;
use std::io::{stdin, BufWriter, Cursor, Write};
use std::path::PathBuf;

use crate::commands::edl::EdlArgs;
use anyhow::{bail, ensure, Result};
use dolby_vision::rpu::utils::parse_rpu_file;

use crate::cmv40::EditRate;
use vtc::{Framerate, Ntsc, Timecode};

#[derive(Debug, Default)]
pub struct EdlConverter {
    frame_index: usize,
    shots: Vec<usize>,
}

impl EdlConverter {
    pub fn convert(args: EdlArgs) -> Result<()> {
        let input = match args.input {
            Some(input) => input,
            None => bail!("No input file provided."),
        };

        ensure!(args.count != Some(0), "Invalid specified frame count.");

        println!("Parsing RPU file...");

        let rpus = parse_rpu_file(input.clone())?;

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

        let mut edl = EdlConverter::default();

        let mut skip_count = args.skip;

        for rpu in rpus {
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }

            if count > 0 {
                if let Some(ref vdr) = rpu.vdr_dm_data {
                    if vdr.scene_refresh_flag == 1 {
                        edl.shots.push(edl.frame_index);
                    }
                }

                edl.frame_index += 1;
                count -= 1;
            }
        }

        if edl.shots.len() == edl.frame_index && edl.frame_index > 1 && !args.force {
            println!(
                "Per-frame rpu detected, no need to generate EDL. Do you want to proceed? (Y/n)"
            );

            let mut input = String::new();
            stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() != "y" {
                println!("Aborted.");
                return Ok(());
            }
        }

        // Last frame
        edl.shots.push(edl.frame_index);

        let edit_rate = EditRate::from(args.rate);

        // We do not need to consider playback time here, so always uses NDF.
        let ntsc_flag = match edit_rate.0[1] {
            1 => Ntsc::None,
            1001 => Ntsc::NonDropFrame,
            _ => unimplemented!("Only /1 or /1001 denom is supported."),
        };

        let frame_rate = Framerate::with_playback(format!("{edit_rate}"), ntsc_flag).unwrap();

        let start_tc_record = Timecode::with_frames(args.start_timecode, frame_rate).unwrap();
        // let start_tc_source = Timecode::with_frames(0, frame_rate).unwrap();

        let mut frame_in = 0;

        let mut buffer_vec = Vec::new();

        for (i, chunk) in edl.shots.chunks(9999).enumerate() {
            let mut edl_buffer = Vec::<u8>::new();
            let mut writer = Cursor::new(&mut edl_buffer);

            // TODO: rename
            write!(
                writer,
                "TITLE: Timeline {} {i}\r\nFCM: NON-DROP FRAME\r\n\r\n",
                input.file_stem().unwrap().to_str().unwrap()
            )?;

            for (j, &shot) in chunk.iter().enumerate() {
                if shot == 0 {
                    continue;
                }

                let frame_out = shot;

                let tc_source_in = Timecode::with_frames(frame_in, frame_rate).unwrap();
                let tc_source_out = Timecode::with_frames(frame_out, frame_rate).unwrap();

                let tc_duration = tc_source_out - tc_source_in;

                let tc_record_in = start_tc_record + tc_source_in;
                let tc_record_out = start_tc_record + tc_source_out;

                let k = j - if i == 0 { 0 } else { 1 };

                // TODO: Transition
                write!(
                    writer,
                    "{:>04}  AX       V     C        {} {} {} {}  \r\n* FROM CLIP NAME: {}\r\n\r\n",
                    k,
                    tc_source_in.timecode(),
                    tc_duration.timecode(),
                    tc_record_in.timecode(),
                    tc_record_out.timecode(),
                    args.clip_name
                )?;

                frame_in = frame_out;
            }

            // println!("{}", String::from_utf8(edl_buffer.clone())?);
            buffer_vec.push(edl_buffer);
        }

        let output = if let Some(output) = args.output {
            output
        } else {
            println!("No output file provided, writing to metadata.edl at current path...");
            "./metadata.edl".into()
        };

        if buffer_vec.len() == 1 {
            let mut output_buffer = BufWriter::new(File::create(output)?);
            write!(
                output_buffer,
                "{}",
                String::from_utf8(buffer_vec[0].clone())?
            )?;
        } else {
            let prefix = output.file_stem().unwrap().to_os_string();
            let extension = if let Some(extension) = output.extension() {
                extension
            } else {
                OsStr::new("edl")
            };

            for (i, buffer) in buffer_vec.iter().enumerate() {
                let suffix_string = format!("_{i}");
                let suffix = OsStr::new(suffix_string.as_str());
                let mut output_name = prefix.clone();
                output_name.extend([suffix, extension]);
                let mut output_buffer = BufWriter::new(File::create(PathBuf::from(output_name))?);
                write!(output_buffer, "{}", String::from_utf8(buffer.clone())?)?;
            }
        }

        Ok(())
    }
}
