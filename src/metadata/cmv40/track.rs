use std::array;

use anyhow::{ensure, Result};
use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlock;
use dolby_vision::rpu::vdr_dm_data::VdrDmData;
use serde::Serialize;

use crate::cmv29::Rate;
use crate::cmv40::display::Characteristics;
use crate::cmv40::Shot;
use crate::display::Chromaticity;
use crate::levels::*;
use crate::MDFType::CMV40;
use crate::{
    cmv29, display, ColorSpace, ColorSpaceEnum, Encoding, EncodingEnum, IntoCMV29, MDFType,
    Primaries, SignalRange, SignalRangeEnum, UUIDv4,
};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Track {
    #[serde(rename = "TrackName")]
    pub track_name: String,
    #[serde(rename = "UniqueID")]
    pub unique_id: UUIDv4,
    #[serde(rename = "EditRate")]
    pub edit_rate: MDFType<EditRate>,
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

impl Track {
    pub fn with_single_vdr(vdr: &VdrDmData) -> Self {
        let level6 = match vdr.get_block(6) {
            Some(ExtMetadataBlock::Level6(b)) => Some(Level6::from(b)),
            _ => None,
        };

        Self {
            // TODO: as option
            track_name: "V1".to_string(),
            unique_id: UUIDv4::new(),
            edit_rate: CMV40(EditRate::default()),
            color_encoding: Default::default(),
            level6,
            plugin_node: vdr.into(),
            shots: None,
        }
    }
}

impl IntoCMV29<cmv29::Track> for Track {
    fn into_cmv29(self) -> cmv29::Track {
        cmv29::Track {
            name: self.track_name,
            unique_id: self.unique_id,
            rate: self.edit_rate.into_inner().into_cmv29(),
            color_encoding: self.color_encoding.into(),
            level6: self.level6,
            plugin_node: self.plugin_node.into(),
            // Source UUID in each shot is not updated yet
            shots: self.shots.into_cmv29(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct EditRate(pub [usize; 2]);

impl EditRate {
    pub fn validate(&self) -> Result<()> {
        ensure!(self.0[0] != 0 && self.0[1] != 0, "Invalid frame rate.");

        Ok(())
    }
}

impl Default for EditRate {
    fn default() -> Self {
        // TODO
        Self([24000, 1001])
    }
}

impl From<Vec<usize>> for EditRate {
    fn from(vec: Vec<usize>) -> Self {
        let mut array = [1; 2];

        vec.iter().enumerate().for_each(|(i, n)| array[i] = *n);

        Self(array)
    }
}

impl IntoIterator for EditRate {
    type Item = usize;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoCMV29<Rate> for EditRate {
    fn into_cmv29(self) -> Rate {
        Rate {
            n: self.0[0],
            d: self.0[1],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TrackPluginNode {
    #[serde(rename = "DVGlobalData")]
    pub dv_global_data: DVGlobalData,
    // Version 5.1.0+
    #[serde(rename = "Level11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level11: Option<Level11>,
    // For Version 4.0.2+, level254 should not be None.
    #[serde(rename = "Level254")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level254: Option<Level254>,
}

impl From<&VdrDmData> for TrackPluginNode {
    fn from(vdr: &VdrDmData) -> Self {
        let level11 = vdr.get_block(11).and_then(|b| match b {
            ExtMetadataBlock::Level11(b) => Some(Level11::from(b)),
            _ => None,
        });

        let level254 = vdr.get_block(254).and_then(|b| match b {
            ExtMetadataBlock::Level254(b) => Some(Level254::from(b)),
            _ => None,
        });

        let mastering_display = display::Characteristics::get_source_or_default(vdr).into();

        Self {
            dv_global_data: DVGlobalData {
                level: 0,
                mastering_display,
                target_displays: None,
            },
            level11,
            level254,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ColorEncoding {
    #[serde(rename = "Primaries")]
    pub primaries: Primaries,
    #[serde(rename = "WhitePoint")]
    pub white_point: MDFType<Chromaticity>,
    #[serde(rename = "PeakBrightness")]
    pub peak_brightness: usize,
    #[serde(rename = "MinimumBrightness")]
    pub minimum_brightness: usize,
    #[serde(rename = "Encoding")]
    pub encoding: Encoding,
    #[serde(rename = "ColorSpace")]
    pub color_space: ColorSpace,
    #[serde(rename = "SignalRange")]
    pub signal_range: SignalRange,
}

// TODO: Default is BT.2020 PQ, should provide other options
impl Default for ColorEncoding {
    fn default() -> Self {
        let p = display::Primaries::get_index_primary(2, false).unwrap_or_default();

        Self {
            primaries: p.into(),
            white_point: CMV40(p.white_point),
            peak_brightness: 10000,
            minimum_brightness: 0,
            encoding: Encoding {
                encoding: EncodingEnum::Pq,
            },
            color_space: ColorSpace {
                color_space: ColorSpaceEnum::Rgb,
            },
            signal_range: SignalRange {
                signal_range: SignalRangeEnum::Computer,
            },
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DVGlobalData {
    // 0
    #[serde(rename = "@level")]
    pub level: usize,
    #[serde(rename = "MasteringDisplay")]
    pub mastering_display: Characteristics,
    #[serde(rename = "TargetDisplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_displays: Option<Vec<Characteristics>>,
}
