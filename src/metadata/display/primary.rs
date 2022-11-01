use std::array;

use dolby_vision::rpu::extension_metadata::blocks::{
    ExtMetadataBlock, ExtMetadataBlockInfo, ExtMetadataBlockLevel9,
};
use dolby_vision::rpu::vdr_dm_data::VdrDmData;

use crate::display::chromaticity::Chromaticity;
use crate::display::PREDEFINED_COLORSPACE_PRIMARIES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Primaries {
    pub red: Chromaticity,
    pub green: Chromaticity,
    pub blue: Chromaticity,
    pub white_point: Chromaticity,
}

impl Primaries {
    pub fn f32_from_rpu_u16(u: u16) -> f32 {
        (match u {
            0..=32767 => u as f32,
            // input value 32768 is undefined, should not happen
            _ => u as f32 - 65536.0,
        }) / 32767.0
    }

    pub fn get_index(&self) -> Option<usize> {
        PREDEFINED_COLORSPACE_PRIMARIES
            .iter()
            .enumerate()
            .find(|(_, p)| Self::from(**p) == *self)
            .map(|(i, _)| i)
    }

    pub fn get_index_primary(index: usize, is_target: bool) -> Option<Self> {
        let index_max = PREDEFINED_COLORSPACE_PRIMARIES.len();
        let index = if index >= index_max || is_target && index > 8 {
            None
        } else {
            Some(index)
        };

        index.map(|index| Primaries::from(PREDEFINED_COLORSPACE_PRIMARIES[index]))
    }
}

impl IntoIterator for Primaries {
    type Item = f32;
    type IntoIter = array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        let mut result = [0.0; 8];

        // We know size is 8
        let vec = [self.red, self.green, self.blue, self.white_point]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        for (i, v) in result.iter_mut().zip(vec) {
            *i = v
        }

        result.into_iter()
    }
}

impl Default for Primaries {
    fn default() -> Self {
        Self::from(PREDEFINED_COLORSPACE_PRIMARIES[0])
    }
}

impl From<[f32; 8]> for Primaries {
    fn from(p: [f32; 8]) -> Self {
        Self {
            red: Chromaticity([p[0], p[1]]),
            green: Chromaticity([p[2], p[3]]),
            blue: Chromaticity([p[4], p[5]]),
            white_point: Chromaticity([p[6], p[7]]),
        }
    }
}

impl From<[u16; 8]> for Primaries {
    fn from(p: [u16; 8]) -> Self {
        let mut result = [0.0f32; 8];

        for (i, j) in result.iter_mut().zip(p) {
            *i = Self::f32_from_rpu_u16(j)
        }

        Primaries::from(result)
    }
}

impl From<&ExtMetadataBlockLevel9> for Primaries {
    fn from(block: &ExtMetadataBlockLevel9) -> Self {
        match block.bytes_size() {
            1 => Primaries::get_index_primary(block.source_primary_index as usize, false)
                .unwrap_or_default(),
            17 => Primaries::from([
                block.source_primary_red_x,
                block.source_primary_red_y,
                block.source_primary_green_x,
                block.source_primary_green_y,
                block.source_primary_blue_x,
                block.source_primary_blue_y,
                block.source_primary_white_x,
                block.source_primary_white_y,
            ]),
            _ => unreachable!(),
        }
    }
}

// For source display
impl From<&VdrDmData> for Primaries {
    fn from(vdr: &VdrDmData) -> Self {
        vdr.get_block(9)
            .and_then(|b| match b {
                ExtMetadataBlock::Level9(b) => Some(Self::from(b)),
                _ => None,
            })
            .unwrap_or_default()
    }
}
