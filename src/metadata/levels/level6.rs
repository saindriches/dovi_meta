use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel6;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Level6 {
    #[serde(rename = "@level")]
    pub level: usize,
    #[serde(rename = "MaxCLL")]
    pub max_cll: usize,
    #[serde(rename = "MaxFALL")]
    pub max_fall: usize,
}

impl Default for Level6 {
    fn default() -> Self {
        Self {
            level: 6,
            max_cll: 0,
            max_fall: 0,
        }
    }
}

impl From<&ExtMetadataBlockLevel6> for Level6 {
    fn from(block: &ExtMetadataBlockLevel6) -> Self {
        Self {
            level: 6,
            max_cll: block.max_content_light_level as usize,
            max_fall: block.max_frame_average_light_level as usize,
        }
    }
}
