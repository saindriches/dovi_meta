use serde::Serialize;

use crate::cmv29::{AlgorithmVersions, Characteristics, Shot};
use crate::display::Chromaticity;
use crate::MDFType::CMV29;
use crate::{
    cmv40, ColorSpace, Encoding, IntoCMV29, Level6, MDFType, Primaries, SignalRange,
    SignalRangeEnum, UUIDv4,
};

#[derive(Debug, Serialize)]
pub struct Track {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "Rate")]
    pub rate: Rate,
    #[serde(rename = "ColorEncoding")]
    pub color_encoding: ColorEncoding,
    #[serde(rename = "Level6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level6: Option<Level6>,
    #[serde(rename = "PluginNode")]
    pub plugin_node: TrackPluginNode,
    #[serde(rename = "Shot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots: Option<Vec<Shot>>,
}

#[derive(Debug, Serialize)]
pub struct Rate {
    #[serde(rename = "n")]
    pub n: usize,
    #[serde(rename = "d")]
    pub d: usize,
}

#[derive(Debug, Serialize)]
pub struct ColorEncoding {
    #[serde(rename = "Primaries")]
    pub primaries: Primaries,
    // Format: f32,f32
    #[serde(rename = "WhitePoint")]
    pub white_point: MDFType<Chromaticity>,
    #[serde(rename = "PeakBrightness")]
    pub peak_brightness: usize,
    #[serde(rename = "MinimumBrightness")]
    pub minimum_brightness: usize,
    #[serde(rename = "Encoding")]
    pub encoding: Encoding,
    #[serde(rename = "BitDepth")]
    pub bit_depth: usize,
    #[serde(rename = "ColorSpace")]
    pub color_space: ColorSpace,
    // FIXME: use usize?
    #[serde(rename = "ChromaFormat")]
    pub chroma_format: String,
    #[serde(rename = "SignalRange")]
    pub signal_range: SignalRange,
}

impl From<cmv40::ColorEncoding> for ColorEncoding {
    fn from(c: cmv40::ColorEncoding) -> Self {
        Self {
            primaries: c.primaries.into_cmv29(),
            white_point: c.white_point.into_cmv29(),
            peak_brightness: c.peak_brightness,
            minimum_brightness: c.minimum_brightness,
            encoding: c.encoding,
            // TODO: as an option?
            bit_depth: 16,
            color_space: c.color_space,
            chroma_format: "444".to_string(),
            signal_range: SignalRange {
                signal_range: SignalRangeEnum::Computer,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrackPluginNode {
    #[serde(rename = "DolbyEDR")]
    pub dolby_edr: TrackDolbyEDR,
}

impl From<cmv40::TrackPluginNode> for TrackPluginNode {
    fn from(t: cmv40::TrackPluginNode) -> Self {
        Self {
            dolby_edr: TrackDolbyEDR {
                algorithm_versions: CMV29(AlgorithmVersions::default()),
                characteristics: Characteristics {
                    level: 0,
                    mastering_display: t.dv_global_data.mastering_display.into_cmv29(),
                    target_displays: t.dv_global_data.target_displays.into_cmv29(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrackDolbyEDR {
    // Format: usize,usize
    #[serde(rename = "AlgorithmVersions")]
    pub algorithm_versions: MDFType<AlgorithmVersions>,
    #[serde(rename = "Characteristics")]
    pub characteristics: Characteristics,
}
