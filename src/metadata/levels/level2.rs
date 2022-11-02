use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel2;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::display::find_target_id;
use crate::f32_from_rpu_u12_with_bias;
use crate::metadata::display::Characteristics;
use crate::metadata::MDFType::*;
use crate::metadata::{IntoCMV29, MDFType};

use super::TrimSixField;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Level2 {
    pub level: u8,
    pub tid: usize,
    // Format: 0 0 0 f32 f32 f32 f32 f32 f32
    pub trim: MDFType<TrimSixField>,
}

impl Serialize for Level2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut trim = self.trim;
        let mut new_trim = [0.0; 9];
        new_trim
            .iter_mut()
            .skip(3)
            .zip(self.trim.into_inner().0)
            .for_each(|(t, s)| *t = s);

        let mut state = serializer.serialize_struct("Level2", 3)?;

        state.serialize_field("level", &self.level)?;
        state.serialize_field("$unflatten=TID", &self.tid)?;
        state.serialize_field("$unflatten=Trim", &trim.with_new_inner(new_trim))?;

        state.end()
    }
}

impl IntoCMV29<Self> for Level2 {
    fn into_cmv29(self) -> Self {
        Self {
            level: 2,
            tid: self.tid,
            trim: self.trim.into_cmv29(),
        }
    }
}

impl Level2 {
    pub fn with_primary_index(block: &ExtMetadataBlockLevel2, primary: Option<usize>) -> Self {
        // identical definition for all negative values, use -1 for v2.0.5+
        let ms_weight = if block.ms_weight < 0 {
            -1.0
        } else {
            f32_from_rpu_u12_with_bias(block.ms_weight as u16)
        };

        let luminance = Characteristics::max_u16_from_rpu_pq_u12(block.target_max_pq);
        let primary = if luminance == 100 {
            1
        } else {
            // P3 D65
            primary.unwrap_or(0)
        };

        // For convenience, use target_max_pq as Level2 custom target display id
        let tid = find_target_id(luminance, primary).unwrap_or(block.target_max_pq as usize);

        let mut trim = TrimSixField([
            f32_from_rpu_u12_with_bias(block.trim_slope),
            f32_from_rpu_u12_with_bias(block.trim_offset),
            f32_from_rpu_u12_with_bias(block.trim_power),
            f32_from_rpu_u12_with_bias(block.trim_chroma_weight),
            f32_from_rpu_u12_with_bias(block.trim_saturation_gain),
            ms_weight,
        ]);

        trim.sop_to_lgg();

        Self {
            level: 2,
            tid,
            trim: CMV40(trim),
        }
    }
}

impl From<&ExtMetadataBlockLevel2> for Level2 {
    fn from(block: &ExtMetadataBlockLevel2) -> Self {
        // P3 D65
        Self::with_primary_index(block, None)
    }
}
