use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel9;
use serde::Serialize;

use crate::MDFType::CMV40;
use crate::{display, MDFType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Level9 {
    pub level: u8,
    // 255
    #[serde(rename = "$unflatten=SourceColorModel")]
    pub source_color_model: u8,
    // Format: f32 f32 f32 f32 f32 f32 f32 f32
    #[serde(rename = "$unflatten=SourceColorPrimary")]
    pub source_color_primary: MDFType<display::Primaries>,
}

impl From<display::Primaries> for Level9 {
    fn from(p: display::Primaries) -> Self {
        Self {
            level: 9,
            source_color_model: 255,
            source_color_primary: CMV40(p),
        }
    }
}

impl From<&ExtMetadataBlockLevel9> for Level9 {
    fn from(block: &ExtMetadataBlockLevel9) -> Self {
        display::Primaries::from(block).into()
    }
}
