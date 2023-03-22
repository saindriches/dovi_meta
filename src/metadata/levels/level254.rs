use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel254;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Level254 {
    #[serde(rename = "@level")]
    pub level: u8,
    #[serde(rename = "DMMode")]
    pub dm_mode: u8,
    #[serde(rename = "DMVersion")]
    pub dm_version: u8,
    // Format: u8 u8
    #[serde(rename = "CMVersion")]
    pub cm_version: String,
}

impl Default for Level254 {
    fn default() -> Self {
        (&ExtMetadataBlockLevel254::cmv402_default()).into()
    }
}

impl From<&ExtMetadataBlockLevel254> for Level254 {
    fn from(block: &ExtMetadataBlockLevel254) -> Self {
        Self {
            level: 254,
            dm_mode: block.dm_mode,
            dm_version: block.dm_version_index,
            // FIXME: Hardcode
            cm_version: "4 1".to_string(),
        }
    }
}
