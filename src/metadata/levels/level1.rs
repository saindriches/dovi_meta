use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel1;
use serde::Serialize;

use crate::metadata::MDFType::*;
use crate::metadata::{IntoCMV29, MDFType};

use super::ImageCharacter;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Level1 {
    #[serde(rename = "@level")]
    pub level: u8,
    #[serde(rename = "ImageCharacter")]
    pub image_character: MDFType<ImageCharacter>,
}

impl From<&ExtMetadataBlockLevel1> for Level1 {
    fn from(block: &ExtMetadataBlockLevel1) -> Self {
        Self {
            level: 1,
            image_character: CMV40(block.into()),
        }
    }
}

impl IntoCMV29<Self> for Level1 {
    fn into_cmv29(self) -> Self {
        Self {
            level: 1,
            image_character: self.image_character.into_cmv29(),
        }
    }
}

impl Default for Level1 {
    fn default() -> Self {
        Self {
            level: 0,
            image_character: CMV40(ImageCharacter([0.0; 3])),
        }
    }
}
