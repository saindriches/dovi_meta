use std::array;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Add;

use dolby_vision::rpu::extension_metadata::blocks::{
    ExtMetadataBlockLevel1, ExtMetadataBlockLevel3,
};

pub use level1::Level1;
pub use level11::Level11;
pub use level2::Level2;
pub use level254::Level254;
pub use level3::Level3;
pub use level5::Level5;
pub use level6::Level6;
pub use level8::Level8;
pub use level9::Level9;

mod level1;
mod level11;
mod level2;
mod level254;
mod level3;
mod level5;
mod level6;
mod level8;
mod level9;

pub const RPU_PQ_MAX: f32 = 4095.0;
// pub const RPU_PQ_OFFSET: f32 = 2048.0;
pub const RPU_U8_BIAS: f32 = 128.0;
pub const RPU_U12_BIAS: f32 = 2048.0;
pub const UHD_WIDTH: usize = 3840;
pub const UHD_HEIGHT: usize = 2160;
pub const UHD_CANVAS: (usize, usize) = (UHD_WIDTH, UHD_HEIGHT);
pub const UHD_AR: f32 = 16.0 / 9.0;

pub fn f32_from_rpu_u12_with_bias(u: u16) -> f32 {
    let u = if u == 4095 { 4096 } else { u };

    (u as f32 - RPU_U12_BIAS) / RPU_U12_BIAS
}

pub fn f32_from_rpu_u8_with_bias(u: u8) -> f32 {
    let u = if u == 255 { 256 } else { u as u16 };

    (u as f32 - RPU_U8_BIAS) / RPU_U8_BIAS
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TrimSixField([f32; 6]);

impl TrimSixField {
    pub fn sop_to_lgg(&mut self) {
        let slope = self.0[0];
        let offset = self.0[1];
        let power = self.0[2];

        let gain = slope + offset;
        let lift = 2.0 * offset / (gain + 2.0);
        let gamma = (4.0 / (power + 2.0) - 2.0).min(1.0);

        self.0[0] = lift.clamp(-1.0, 1.0);
        self.0[1] = gain.clamp(-1.0, 1.0);
        self.0[2] = gamma;
    }
}

impl IntoIterator for TrimSixField {
    type Item = f32;
    type IntoIter = array::IntoIter<Self::Item, 6>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageCharacter([f32; 3]);

impl ImageCharacter {
    pub fn new() -> Self {
        Self([0.0; 3])
    }
}

impl Add for ImageCharacter {
    type Output = ImageCharacter;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        let Self(lhs) = self;
        let Self(rhs) = rhs;

        for ((i, a), b) in result.0.iter_mut().zip(&lhs).zip(&rhs) {
            *i = a + b;
        }

        result
    }
}

impl From<&ExtMetadataBlockLevel1> for ImageCharacter {
    fn from(block: &ExtMetadataBlockLevel1) -> Self {
        Self([
            block.min_pq as f32 / RPU_PQ_MAX,
            block.avg_pq as f32 / RPU_PQ_MAX,
            block.max_pq as f32 / RPU_PQ_MAX,
        ])
    }
}

impl From<&ExtMetadataBlockLevel3> for ImageCharacter {
    fn from(block: &ExtMetadataBlockLevel3) -> Self {
        Self([
            f32_from_rpu_u12_with_bias(block.min_pq_offset),
            f32_from_rpu_u12_with_bias(block.avg_pq_offset),
            f32_from_rpu_u12_with_bias(block.max_pq_offset),
        ])
    }
}

impl IntoIterator for ImageCharacter {
    type Item = f32;
    type IntoIter = array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AspectRatio([f32; 2]);

impl IntoIterator for AspectRatio {
    type Item = f32;
    type IntoIter = array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for AspectRatio {
    fn eq(&self, other: &Self) -> bool {
        let self_ar = self.0[1] / self.0[0];
        let other_ar = other.0[1] / other.0[0];

        self_ar.to_bits() == other_ar.to_bits()
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for AspectRatio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_ar = self.0[1] / self.0[0];
        let other_ar = other.0[1] / other.0[0];

        self_ar.partial_cmp(&other_ar)
    }
}

// NaN should not happen for AspectRatio
impl Ord for AspectRatio {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Eq for AspectRatio {}

impl Hash for AspectRatio {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|f| f.to_bits().hash(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sop_to_lgg() {
        let mut trim = TrimSixField([
            f32_from_rpu_u12_with_bias(4095),
            f32_from_rpu_u12_with_bias(0),
            0.0,
            0.0,
            0.0,
            0.0,
        ]);
        trim.sop_to_lgg();

        assert_eq!(trim.0, [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
}
