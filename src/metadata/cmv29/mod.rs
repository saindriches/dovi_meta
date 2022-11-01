use serde::Serialize;
use std::array;

pub use display::*;
pub use frame::*;
pub use shot::*;
pub use track::*;

use crate::{RevisionHistory, UUIDv4, Version};

mod display;
mod frame;
mod shot;
mod track;

#[derive(Debug, Serialize)]
pub struct DolbyLabsMDF {
    pub version: Version,
    #[serde(rename = "xmlns:xsd")]
    pub xmlns_xsd: String,
    #[serde(rename = "xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "SourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_list: Option<SourceList>,
    #[serde(rename = "RevisionHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history: Option<RevisionHistory>,
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Outputs>,
}

#[derive(Debug, Serialize)]
pub struct SourceList {
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
}

// TODO: Some other fields are available here
#[derive(Debug, Serialize)]
pub struct Source {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "$unflatten=In")]
    pub in_: usize,
    #[serde(rename = "$unflatten=Duration")]
    pub duration: usize,
}

#[derive(Debug, Serialize)]
pub struct Outputs {
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

impl Outputs {
    pub fn get_source_list(&self) -> Option<SourceList> {
        if let Some(sources) = self.outputs.as_ref().map(|outputs| {
            outputs
                .iter()
                .filter_map(|output| {
                    output
                        .video
                        .tracks
                        .last()
                        .and_then(|track| track.shots.as_ref())
                        .and_then(|shots| shots.last())
                        .map(|shot| {
                            let record = shot.record.clone();
                            let source = shot.source.clone();

                            let duration = record.in_ + record.duration;
                            let unique_id = source.parent_id;

                            Source {
                                type_: "Video".to_string(),
                                unique_id,
                                in_: 0,
                                duration,
                            }
                        })
                })
                .collect::<Vec<_>>()
        }) {
            let source_list = SourceList {
                sources: Some(sources),
            };

            Some(source_list)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub name: String,
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "$unflatten=NumberVideoTracks")]
    pub number_video_tracks: usize,
    #[serde(rename = "$unflatten=NumberAudioTracks")]
    pub number_audio_tracks: usize,
    #[serde(rename = "$unflatten=CanvasAspectRatio")]
    pub canvas_aspect_ratio: f32,
    #[serde(rename = "$unflatten=ImageAspectRatio")]
    pub image_aspect_ratio: f32,
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Debug, Serialize)]
pub struct Video {
    #[serde(rename = "Track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Copy)]
pub struct AlgorithmVersions([usize; 2]);

impl Default for AlgorithmVersions {
    fn default() -> Self {
        Self([2, 1])
    }
}

impl IntoIterator for AlgorithmVersions {
    type Item = usize;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
