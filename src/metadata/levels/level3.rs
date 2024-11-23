use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel3;
use serde::Serialize;

use crate::MDFType::CMV40;
use crate::{ImageCharacter, MDFType};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Level3 {
    #[serde(rename = "@level")]
    pub level: u8,
    // Format: f32 f32 f32
    #[serde(rename = "L1Offset")]
    pub l1_offset: MDFType<ImageCharacter>,
}

impl Default for Level3 {
    fn default() -> Self {
        Self {
            level: 3,
            l1_offset: Default::default(),
        }
    }
}

impl From<&ExtMetadataBlockLevel3> for Level3 {
    fn from(block: &ExtMetadataBlockLevel3) -> Self {
        Self {
            level: 3,
            l1_offset: CMV40(ImageCharacter::from(block)),
        }
    }
}
