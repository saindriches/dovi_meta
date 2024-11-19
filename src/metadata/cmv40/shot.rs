use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlock;
use dolby_vision::rpu::vdr_dm_data::VdrDmData;
use serde::Serialize;

use crate::cmv40::Frame;
use crate::levels::*;
use crate::{cmv29, IntoCMV29, UUIDv4};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Shot {
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "Record")]
    pub record: Record,
    #[serde(rename = "PluginNode")]
    pub plugin_node: ShotPluginNode,
    #[serde(rename = "Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<Frame>>,
}

impl Shot {
    pub fn update_record(&mut self, index: Option<usize>, duration_override: Option<usize>) {
        match index {
            Some(index) => {
                self.record.in_ = index;
                self.record.duration = 1;
            }
            None => {
                // FIXME: dirty
                self.record.duration += 1;
            }
        }

        if let Some(duration) = duration_override {
            self.record.duration = duration + 1;
        }
    }

    pub fn with_canvas(vdr: &VdrDmData, canvas: (usize, usize)) -> Self {
        Self {
            unique_id: UUIDv4::new(),
            record: Default::default(),
            plugin_node: ShotPluginNode::with_canvas(vdr, canvas),
            frames: None,
        }
    }

    pub fn append_metadata(&mut self, other: &Self) {
        if self.frames.is_none() && self.plugin_node != other.plugin_node {
            self.frames = Some(Vec::new());
        }

        // Always parse per-frame metadata until next shot
        if let Some(ref mut frames) = self.frames {
            let offset = self.record.duration - 1;
            let new_frame = Frame::with_offset(other, offset);
            frames.push(new_frame);
        }
    }
}

impl From<&VdrDmData> for Shot {
    fn from(vdr: &VdrDmData) -> Self {
        Self::with_canvas(vdr, UHD_CANVAS)
    }
}

impl IntoCMV29<cmv29::Shot> for Shot {
    fn into_cmv29(self) -> cmv29::Shot {
        cmv29::Shot {
            unique_id: self.unique_id,
            source: cmv29::ShotSource::default(),
            record: self.record.into(),
            plugin_node: self.plugin_node.dv_dynamic_data.into(),
            frames: self.frames.into_cmv29(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ShotPluginNode {
    #[serde(rename = "DVDynamicData")]
    pub dv_dynamic_data: DVDynamicData,
    // Version 5.1.0+
    #[serde(rename = "Level11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level11: Option<Level11>,
}

impl ShotPluginNode {
    fn with_canvas(vdr: &VdrDmData, canvas: (usize, usize)) -> Self {
        let level11 = vdr.get_block(11).and_then(|b| match b {
            ExtMetadataBlock::Level11(b) => Some(Level11::from(b)),
            _ => None,
        });

        Self {
            dv_dynamic_data: DVDynamicData::with_canvas(vdr, canvas),
            level11,
        }
    }
}

impl From<&VdrDmData> for ShotPluginNode {
    fn from(vdr: &VdrDmData) -> Self {
        Self::with_canvas(vdr, UHD_CANVAS)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct DVDynamicData {
    #[serde(rename = "Level1")]
    pub level1: Level1,
    #[serde(rename = "Level2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: Option<Vec<Level2>>,
    #[serde(rename = "Level3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level3: Option<Level3>,
    #[serde(rename = "Level5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level5: Option<Level5>,
    #[serde(rename = "Level8")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level8: Option<Vec<Level8>>,
    #[serde(rename = "Level9")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level9: Option<Level9>,
}

impl DVDynamicData {
    pub fn with_canvas(vdr: &VdrDmData, canvas: (usize, usize)) -> Self {
        let level1 = if let Some(ExtMetadataBlock::Level1(block)) = vdr.get_block(1) {
            Level1::from(block)
        } else {
            Level1::default()
        };

        let mut primary = None;

        let level9 = vdr.get_block(9).and_then(|b| match b {
            ExtMetadataBlock::Level9(b) => {
                primary = Some(b.source_primary_index as usize);
                Some(Level9::from(b))
            }
            _ => None,
        });

        let level2 = vdr
            .level_blocks_iter(2)
            .map(|b| match b {
                ExtMetadataBlock::Level2(b) => Some(Level2::with_primary_index(b, primary)),
                _ => None,
            })
            .collect::<Option<Vec<_>>>();

        let level3 = vdr.get_block(3).and_then(|b| match b {
            ExtMetadataBlock::Level3(b) => Some(Level3::from(b)),
            _ => None,
        });

        let level5 = {
            let b = vdr.get_block(5).and_then(|b| match b {
                ExtMetadataBlock::Level5(b) => Some(b),
                _ => None,
            });

            Some(Level5::with_canvas(b, canvas))
        };

        let level8 = vdr
            .level_blocks_iter(8)
            .map(|b| match b {
                ExtMetadataBlock::Level8(b) => Some(Level8::from(b)),
                _ => None,
            })
            .collect::<Option<Vec<_>>>();

        Self {
            level1,
            level2,
            level3,
            level5,
            level8,
            level9,
        }
    }
}

impl From<&VdrDmData> for DVDynamicData {
    fn from(vdr: &VdrDmData) -> Self {
        Self::with_canvas(vdr, UHD_CANVAS)
    }
}

// TODO: Start duration is 1
#[derive(Debug, Clone, Default, Serialize)]
pub struct Record {
    #[serde(rename = "In")]
    pub in_: usize,
    #[serde(rename = "Duration")]
    pub duration: usize,
}
