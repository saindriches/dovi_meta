use std::hash::{Hash, Hasher};
use std::intrinsics::transmute;

use dolby_vision::rpu::extension_metadata::blocks::{
    ExtMetadataBlock, ExtMetadataBlockInfo, ExtMetadataBlockLevel10, ExtMetadataBlockLevel2,
    ExtMetadataBlockLevel8,
};
use dolby_vision::rpu::vdr_dm_data::VdrDmData;
use itertools::Itertools;

use crate::display::{PREDEFINED_MASTERING_DISPLAYS, PREDEFINED_TARGET_DISPLAYS, RPU_PQ_MAX};
use crate::metadata::display::primary::Primaries;
use crate::{display, Encoding, EncodingEnum};

#[derive(Debug, Clone)]
pub struct Characteristics {
    pub name: String,
    pub id: usize,
    pub primary_index: usize,
    pub primaries: Primaries,
    pub peak_brightness: usize,
    pub minimum_brightness: f32,
    pub encoding: Encoding,
    pub diagonal_size: usize,
}

impl PartialEq for Characteristics {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.id == other.id
            && self.primary_index == other.primary_index
            && self.primaries == other.primaries
            && self.peak_brightness == other.peak_brightness
            && self.minimum_brightness.to_bits() == other.minimum_brightness.to_bits()
            && self.encoding == other.encoding
            && self.diagonal_size == other.diagonal_size
    }
}

impl Eq for Characteristics {}

impl Hash for Characteristics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.id.hash(state);
        self.primary_index.hash(state);
        self.primaries.hash(state);
        self.peak_brightness.hash(state);
        self.minimum_brightness.to_bits().hash(state);
        self.encoding.hash(state);
        self.diagonal_size.hash(state);
    }
}

impl Characteristics {
    pub fn update_name(&mut self) {
        let color_model = match self.primary_index {
            0 => "P3, D65",
            1 => "BT.709",
            2 => "BT.2020",
            5 => "P3, DCI",
            9 => "WCG, D65",
            _ => "Custom",
        };

        let eotf = match self.encoding.encoding {
            EncodingEnum::Pq => "ST.2084",
            EncodingEnum::Linear => "Linear",
            EncodingEnum::GammaBT1886 => "BT.1886",
            EncodingEnum::GammaDCI => "Gamma2.6",
            EncodingEnum::Gamma22 => "Gamma2.2",
            EncodingEnum::Gamma24 => "Gamma2.4",
            EncodingEnum::Hlg => "HLG",
        };

        self.name = format!(
            "{}-nits, {}, {}, Full",
            self.peak_brightness, color_model, eotf
        )
    }

    pub fn max_u16_from_rpu_pq_u12(u: u16) -> usize {
        match u {
            // Common cases
            2081 => 100,
            2851 => 600,
            3079 => 1000,
            3696 => 4000,
            _ => {
                let n = display::pq2l(u as f32 / RPU_PQ_MAX).round();
                // smooth large values
                if n > 500.0 {
                    (n / 50.0 + 1.0) as usize * 50
                } else {
                    n as usize
                }
            }
        }
    }

    fn min_f32_from_rpu_pq_u12(u: u16) -> f32 {
        match u {
            // Common cases
            0 => 0.0,
            7 => 0.0001,
            26 => 0.001,
            62 => 0.005,
            _ => display::pq2l(u as f32 / RPU_PQ_MAX),
        }
    }

    fn get_primary_target(block: &ExtMetadataBlockLevel2, primary: Primaries) -> Self {
        let max_luminance = Self::max_u16_from_rpu_pq_u12(block.target_max_pq);

        let primary_index = if let Some(primary) = primary.get_index() {
            if max_luminance == 100 {
                1
            } else {
                primary
            }
        } else {
            0
        };

        if let Some(target) =
            Self::get_display(PREDEFINED_TARGET_DISPLAYS, max_luminance, primary_index)
        {
            target
        } else {
            let mut target = Self {
                id: block.target_max_pq as usize,
                primary_index,
                primaries: primary,
                peak_brightness: max_luminance,
                minimum_brightness: 0.0,
                ..Default::default()
            };

            target.update_name();
            target
        }
    }

    fn get_target(block: &ExtMetadataBlockLevel8) -> Option<Self> {
        let index = block.target_display_index as usize;

        PREDEFINED_TARGET_DISPLAYS
            .iter()
            .find(|d| (**d)[0] == index)
            .map(|d| Self::from(*d))
    }

    pub fn get_targets(vdr: &VdrDmData) -> Option<Vec<Self>> {
        let mut targets = Vec::new();

        let primary = Primaries::from(vdr);

        vdr.level_blocks_iter(10).for_each(|b| {
            if let ExtMetadataBlock::Level10(b) = b {
                let d = Self::from(b);
                targets.push(d);
            }
        });

        vdr.level_blocks_iter(8).for_each(|b| {
            if let ExtMetadataBlock::Level8(b) = b {
                if let Some(d) = Self::get_target(b) {
                    targets.push(d)
                }
            }
        });

        vdr.level_blocks_iter(2).for_each(|b| {
            if let ExtMetadataBlock::Level2(b) = b {
                targets.push(Self::get_primary_target(b, primary))
            }
        });

        let targets = targets
            .into_iter()
            .unique()
            .sorted_by_key(|c| c.id)
            .collect::<Vec<_>>();

        if targets.is_empty() {
            None
        } else {
            Some(targets)
        }
    }

    pub fn default_source() -> Self {
        Self::from(PREDEFINED_MASTERING_DISPLAYS[0])
    }

    pub fn get_source_or_default(vdr: &VdrDmData) -> Self {
        let primary = Primaries::from(vdr);
        let primary_index = primary.get_index().unwrap_or(0);

        // Prefer level 6 metadata
        let max_luminance = match vdr.get_block(6) {
            Some(ExtMetadataBlock::Level6(b)) => b.max_display_mastering_luminance as usize,
            _ => Characteristics::max_u16_from_rpu_pq_u12(vdr.source_max_pq),
        };

        if let Some(source) =
            Self::get_display(PREDEFINED_MASTERING_DISPLAYS, max_luminance, primary_index)
        {
            source
        } else {
            let mut source = Self::default_source();

            if vdr.get_block(254).is_some() {
                // Custom mastering display for CM v4.0
                // For convenience, use source_max_pq as custom mastering display id
                source.id = vdr.source_max_pq as usize;
                source.primaries = primary;

                source.primary_index = if primary.get_index().is_none() {
                    // Random invalid value
                    255
                } else {
                    primary_index
                };

                // BT.709 BT.1886
                if primary_index == 1 {
                    source.encoding = Encoding {
                        encoding: EncodingEnum::GammaBT1886,
                    };
                    source.peak_brightness = 100;
                    // Default source (4000-nit) min_brightness is 0.005-nit
                }

                source.update_name();
            }

            source
        }
    }

    /*pub fn update_luminance_range_with_l6_block(&mut self, block: &ExtMetadataBlockLevel6) {
        self.peak_brightness = block.max_display_mastering_luminance as usize;
        self.minimum_brightness = block.min_display_mastering_luminance as f32 / RPU_L6_MIN_FACTOR;
    }*/

    fn get_display(list: &[[usize; 6]], max_luminance: usize, primary: usize) -> Option<Self> {
        list.iter()
            .find(|d| (**d)[2] == max_luminance && (**d)[1] == primary)
            .map(|d| Self::from(*d))
    }
}

impl Default for Characteristics {
    fn default() -> Self {
        Self::from(PREDEFINED_TARGET_DISPLAYS[0])
    }
}

impl From<[usize; 6]> for Characteristics {
    fn from(input: [usize; 6]) -> Self {
        let mut result = Self {
            name: String::new(),
            id: input[0],
            primary_index: input[1],
            primaries: Primaries::get_index_primary(input[1], true).unwrap_or_default(),
            peak_brightness: input[2],
            minimum_brightness: Self::min_f32_from_rpu_pq_u12(input[3] as u16),
            // :(
            encoding: Encoding {
                encoding: unsafe { transmute::<usize, EncodingEnum>(input[4]) },
            },
            // TODO
            diagonal_size: 42,
        };

        result.update_name();
        result
    }
}

impl From<&ExtMetadataBlockLevel10> for Characteristics {
    fn from(block: &ExtMetadataBlockLevel10) -> Self {
        let mut result = Self {
            id: block.target_display_index as usize,
            primary_index: block.target_primary_index as usize,
            primaries: match block.bytes_size() {
                21 => Primaries::from([
                    block.target_primary_red_x,
                    block.target_primary_red_y,
                    block.target_primary_green_x,
                    block.target_primary_green_y,
                    block.target_primary_blue_x,
                    block.target_primary_blue_y,
                    block.target_primary_white_x,
                    block.target_primary_white_y,
                ]),
                5 => Primaries::get_index_primary(block.target_primary_index as usize, true)
                    .unwrap_or_default(),
                _ => unreachable!(),
            },
            peak_brightness: Self::max_u16_from_rpu_pq_u12(block.target_max_pq),
            minimum_brightness: Self::min_f32_from_rpu_pq_u12(block.target_min_pq),
            encoding: Encoding {
                encoding: EncodingEnum::Pq,
            },
            diagonal_size: 42,
            ..Default::default()
        };

        result.update_name();
        result
    }
}
