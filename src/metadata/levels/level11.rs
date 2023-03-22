use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel11;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Level11 {
    #[serde(rename = "@level")]
    pub level: u8,
    #[serde(rename = "ContentType")]
    pub content_type: u8,
    #[serde(rename = "IntendedWhitePoint")]
    pub intended_white_point: u8,
    // FIXME: Rename
    #[serde(rename = "ExtensionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_properties: Option<u8>,
}

impl Default for Level11 {
    fn default() -> Self {
        Self {
            level: 11,
            content_type: 1, // Movies
            intended_white_point: 0,
            extension_properties: None,
        }
    }
}

impl From<&ExtMetadataBlockLevel11> for Level11 {
    fn from(block: &ExtMetadataBlockLevel11) -> Self {
        Self {
            level: 11,
            content_type: block.content_type,
            intended_white_point: block.whitepoint,
            // TODO: byte3?
            extension_properties: None,
        }
    }
}
