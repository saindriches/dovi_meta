use serde::Serialize;

use crate::cmv40::{DVDynamicData, Shot};
use crate::{cmv29, IntoCMV29, UUIDv4};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Frame {
    #[serde(rename = "EditOffset")]
    pub edit_offset: usize,
    #[serde(rename = "DVDynamicData")]
    pub dv_dynamic_data: DVDynamicData,
}

impl Frame {
    pub fn with_offset(shot: &Shot, offset: usize) -> Self {
        let mut dv_dynamic_data = shot.plugin_node.dv_dynamic_data.clone();
        // Remove Level 9 in per-frame metadata
        dv_dynamic_data.level9 = None;
        Self {
            edit_offset: offset,
            dv_dynamic_data,
        }
    }
}

impl From<&Shot> for Frame {
    fn from(shot: &Shot) -> Self {
        Self::with_offset(shot, 0)
    }
}

impl IntoCMV29<cmv29::Frame> for Frame {
    fn into_cmv29(self) -> cmv29::Frame {
        cmv29::Frame {
            unique_id: UUIDv4::new(),
            edit_offset: self.edit_offset,
            plugin_node: self.dv_dynamic_data.into(),
        }
    }
}
