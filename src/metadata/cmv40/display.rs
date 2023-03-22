use serde::Serialize;

use crate::cmv29::CharacteristicsLegacy;
use crate::display::Chromaticity;
use crate::MDFType::CMV40;
use crate::{
    display, ApplicationType, ColorSpace, ColorSpaceEnum, Encoding, IntoCMV29, MDFType, Primaries,
    SignalRange, SignalRangeEnum,
};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Characteristics {
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
    #[serde(rename = "EOTF")]
    pub eotf: Encoding,
    #[serde(rename = "DiagonalSize")]
    pub diagonal_size: usize,
    // Version 5.0.0+
    #[serde(rename = "ApplicationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<ApplicationType>,
}

impl From<display::Characteristics> for Characteristics {
    fn from(d: display::Characteristics) -> Self {
        Self {
            id: d.id,
            name: d.name,
            primaries: d.primaries.into(),
            white_point: CMV40(d.primaries.white_point),
            peak_brightness: d.peak_brightness,
            minimum_brightness: d.minimum_brightness,
            eotf: d.encoding,
            diagonal_size: d.diagonal_size,
            application_type: None,
        }
    }
}

impl IntoCMV29<CharacteristicsLegacy> for Characteristics {
    fn into_cmv29(self) -> CharacteristicsLegacy {
        CharacteristicsLegacy {
            level: 0,
            id: self.id,
            name: self.name,
            primaries: self.primaries.into_cmv29(),
            white_point: self.white_point.into_cmv29(),
            peak_brightness: self.peak_brightness,
            minimum_brightness: self.minimum_brightness,
            diagonal_size: self.diagonal_size,
            encoding: self.eotf,
            bit_depth: 16,
            color_space: ColorSpace {
                color_space: ColorSpaceEnum::Rgb,
            },
            signal_range: SignalRange {
                signal_range: SignalRangeEnum::Computer,
            },
        }
    }
}
