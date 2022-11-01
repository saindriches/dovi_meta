use serde::Serialize;

use crate::display::Chromaticity;
use crate::{ColorSpace, Eotf, MDFType, Primaries, SignalRange};

#[derive(Debug, Serialize)]
pub struct Characteristics {
    // 0
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
    pub level: usize,
    #[serde(rename = "$unflatten=ID")]
    pub id: usize,
    #[serde(rename = "$unflatten=Name")]
    pub name: String,
    #[serde(rename = "Primaries")]
    pub primaries: Primaries,
    #[serde(rename = "$unflatten=WhitePoint")]
    pub white_point: MDFType<Chromaticity>,
    #[serde(rename = "$unflatten=PeakBrightness")]
    pub peak_brightness: usize,
    #[serde(rename = "$unflatten=MinimumBrightness")]
    pub minimum_brightness: f32,
    #[serde(rename = "$unflatten=DiagonalSize")]
    pub diagonal_size: usize,
    #[serde(rename = "$unflatten=Encoding")]
    pub encoding: Eotf,
    #[serde(rename = "$unflatten=BitDepth")]
    pub bit_depth: usize,
    #[serde(rename = "$unflatten=ColorSpace")]
    pub color_space: ColorSpace,
    #[serde(rename = "$unflatten=SignalRange")]
    pub signal_range: SignalRange,
}
