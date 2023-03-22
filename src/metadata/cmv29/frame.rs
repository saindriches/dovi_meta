use serde::Serialize;

use crate::cmv29::ShotPluginNode;
use crate::UUIDv4;

#[derive(Debug, Serialize)]
pub struct Frame {
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "EditOffset")]
    pub edit_offset: usize,
    #[serde(rename = "PluginNode")]
    pub plugin_node: ShotPluginNode,
}
