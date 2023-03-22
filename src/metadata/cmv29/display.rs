use serde::Serialize;

use crate::display::Chromaticity;
use crate::{ColorSpace, Encoding, MDFType, Primaries, SignalRange};

#[derive(Debug, Serialize)]
pub struct Characteristics {
    // 0
    #[serde(rename = "@level")]
    pub level: usize,
    #[serde(rename = "MasteringDisplay")]
    pub mastering_display: CharacteristicsLegacy,
    #[serde(rename = "TargetDisplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_displays: Option<Vec<CharacteristicsLegacy>>,
}

#[derive(Debug, Serialize)]
pub struct CharacteristicsLegacy {
    // 0
    #[serde(rename = "@level")]
    pub level: usize,
    #[serde(rename = "ID")]
    pub id: usize,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Primaries")]
    pub primaries: Primaries,
    #[serde(rename = "WhitePoint")]
    pub white_point: MDFType<Chromaticity>,
    #[serde(rename = "PeakBrightness")]
    pub peak_brightness: usize,
    #[serde(rename = "MinimumBrightness")]
    pub minimum_brightness: f32,
    #[serde(rename = "DiagonalSize")]
    pub diagonal_size: usize,
    #[serde(rename = "Encoding")]
    pub encoding: Encoding,
    #[serde(rename = "BitDepth")]
    pub bit_depth: usize,
    #[serde(rename = "ColorSpace")]
    pub color_space: ColorSpace,
    #[serde(rename = "SignalRange")]
    pub signal_range: SignalRange,
}
