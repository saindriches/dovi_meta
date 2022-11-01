use anyhow::{Context, Result};
use serde::Serialize;

pub use display::*;
pub use frame::*;
pub use shot::*;
pub use track::*;

use crate::XMLVersion::{V402, V510};
use crate::{
    cmv29, ApplicationType, IntoCMV29, Level5, RevisionHistory, UUIDv4, Version, XMLVersion,
    CMV40_MIN_VERSION, UHD_AR,
};

mod display;
mod frame;
mod shot;
mod track;

#[derive(Debug, Serialize)]
pub struct DolbyLabsMDF {
    pub xmlns: String,
    #[serde(rename = "$unflatten=Version")]
    pub version: Version,
    #[serde(rename = "RevisionHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history: Option<RevisionHistory>,
    #[serde(rename = "Outputs")]
    pub outputs: Outputs,
}

impl DolbyLabsMDF {
    pub fn with_single_output(output: Output) -> Result<Self> {
        let mut output = output;

        let has_level11 = output
            .video
            .tracks
            .first()
            .map(|track| track.plugin_node.level11.is_some())
            .context("No track in output.")?;

        let version: Version = match has_level11 {
            true => V510,
            false => V402,
        }
        .into();

        if version > CMV40_MIN_VERSION {
            output.video.tracks.iter_mut().for_each(|track| {
                track
                    .plugin_node
                    .dv_global_data
                    .mastering_display
                    .application_type = Some(ApplicationType::All);

                if let Some(ds) = track.plugin_node.dv_global_data.target_displays.as_mut() {
                    ds.iter_mut()
                        .for_each(|d| d.application_type = Some(ApplicationType::Home))
                }
            });
        }

        Ok(Self {
            xmlns: version.get_dolby_xmlns(),
            version,
            revision_history: Some(RevisionHistory::new()),
            outputs: Outputs {
                outputs: vec![output],
            },
        })
    }
}

impl IntoCMV29<cmv29::DolbyLabsMDF> for DolbyLabsMDF {
    fn into_cmv29(self) -> cmv29::DolbyLabsMDF {
        let outputs = self.outputs.into_cmv29();

        cmv29::DolbyLabsMDF {
            version: XMLVersion::V205.into(),
            xmlns_xsd: "http://www.w3.org/2001/XMLSchema".to_string(),
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            source_list: outputs.get_source_list(),
            revision_history: self.revision_history,
            outputs: Some(outputs),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Outputs {
    #[serde(rename = "Output")]
    pub outputs: Vec<Output>,
}

impl IntoCMV29<cmv29::Outputs> for Outputs {
    fn into_cmv29(self) -> cmv29::Outputs {
        cmv29::Outputs {
            outputs: Some(self.outputs.into_cmv29()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Output {
    #[serde(rename = "$unflatten=CompositionName")]
    pub composition_name: String,
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "$unflatten=NumberVideoTracks")]
    pub number_video_tracks: usize,
    #[serde(rename = "$unflatten=CanvasAspectRatio")]
    pub canvas_aspect_ratio: f32,
    #[serde(rename = "$unflatten=ImageAspectRatio")]
    pub image_aspect_ratio: f32,
    #[serde(rename = "Video")]
    pub video: Video,
}

impl IntoCMV29<cmv29::Output> for Output {
    fn into_cmv29(self) -> cmv29::Output {
        let mut video = self.video.into_cmv29();
        let parent_id = UUIDv4::new();
        video.tracks.iter_mut().for_each(|track| {
            track.shots.iter_mut().for_each(|shots| {
                shots.iter_mut().for_each(|shot| {
                    shot.source.parent_id = parent_id.clone();
                })
            })
        });

        cmv29::Output {
            name: self.composition_name,
            unique_id: self.unique_id,
            number_video_tracks: self.number_video_tracks,
            number_audio_tracks: 0,
            canvas_aspect_ratio: self.canvas_aspect_ratio,
            image_aspect_ratio: self.image_aspect_ratio,
            video,
        }
    }
}

impl Output {
    pub fn with_level5(track: Track, level5: Option<Level5>) -> Self {
        let (canvas_aspect_ratio, image_aspect_ratio) = if let Some(level5) = level5 {
            level5.get_ar()
        } else {
            // Should not happen
            (UHD_AR, UHD_AR)
        };

        Self {
            composition_name: "Timeline".to_string(),
            unique_id: UUIDv4::new(),
            number_video_tracks: 1,
            canvas_aspect_ratio,
            image_aspect_ratio,
            video: Video {
                tracks: vec![track],
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Video {
    #[serde(rename = "Track")]
    pub tracks: Vec<Track>,
}

impl IntoCMV29<cmv29::Video> for Video {
    fn into_cmv29(self) -> cmv29::Video {
        cmv29::Video {
            tracks: self.tracks.into_cmv29(),
        }
    }
}
