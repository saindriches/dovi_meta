use serde::Serialize;

use crate::cmv29::{Frame, Source};
use crate::cmv40::DVDynamicData;
use crate::{cmv40, IntoCMV29, Level1, Level2, Level5, UUIDv4};

#[derive(Debug, Serialize)]
pub struct Shot {
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "Source")]
    pub source: ShotSource,
    #[serde(rename = "Record")]
    pub record: Record,
    #[serde(rename = "PluginNode")]
    pub plugin_node: ShotPluginNode,
    #[serde(rename = "Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<Frame>>,
}

// CMv2.9 only
#[derive(Debug, Clone, Default, Serialize)]
pub struct ShotSource {
    #[serde(rename = "ParentID")]
    pub parent_id: UUIDv4,
    #[serde(rename = "In")]
    pub in_: usize,
}

impl From<Source> for ShotSource {
    fn from(s: Source) -> Self {
        Self {
            parent_id: s.unique_id,
            in_: s.in_,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    #[serde(rename = "In")]
    pub in_: usize,
    #[serde(rename = "Duration")]
    pub duration: usize,
}

impl From<cmv40::Record> for Record {
    fn from(record: cmv40::Record) -> Self {
        Self {
            in_: record.in_,
            duration: record.duration,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ShotPluginNode {
    #[serde(rename = "DolbyEDR")]
    pub level1: Level1,
    #[serde(rename = "DolbyEDR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: Option<Vec<Level2>>,
    #[serde(rename = "DolbyEDR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level5: Option<Level5>,
}

impl From<DVDynamicData> for ShotPluginNode {
    fn from(data: DVDynamicData) -> Self {
        Self {
            level1: data.level1.into_cmv29(),
            level2: data.level2.into_cmv29(),
            level5: data.level5.into_cmv29(),
        }
    }
}
