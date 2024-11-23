use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel8;
use serde::Serialize;

use super::TrimSixField;
use crate::metadata::WithTid;
use crate::MDFType;
use crate::MDFType::CMV40;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Level8 {
    #[serde(rename = "@level")]
    pub level: u8,
    #[serde(rename = "TID")]
    pub tid: u8,
    // Format: f32 f32 f32 f32 f32 f32
    #[serde(rename = "L8Trim")]
    pub l8_trim: MDFType<TrimSixField>,
    #[serde(rename = "MidContrastBias")]
    pub mid_contrast_bias: f32,
    #[serde(rename = "HighlightClipping")]
    pub highlight_clipping: f32,
    // Format: f32 f32 f32 f32 f32 f32
    #[serde(rename = "SaturationVectorField")]
    pub sat_vector_field: MDFType<TrimSixField>,
    // Format: f32 f32 f32 f32 f32 f32
    #[serde(rename = "HueVectorField")]
    pub hue_vector_field: MDFType<TrimSixField>,
}

impl WithTid for Level8 {
    fn tid(&self) -> usize {
        self.tid as usize
    }

    fn with_tid(tid: usize) -> Self {
        Self {
            level: 8,
            tid: tid as u8,
            l8_trim: Default::default(),
            mid_contrast_bias: 0.0,
            highlight_clipping: 0.0,
            sat_vector_field: Default::default(),
            hue_vector_field: Default::default(),
        }
    }
}

impl From<&ExtMetadataBlockLevel8> for Level8 {
    fn from(block: &ExtMetadataBlockLevel8) -> Self {
        let mut trim = TrimSixField([
            crate::f32_from_rpu_u12_with_bias(block.trim_slope),
            crate::f32_from_rpu_u12_with_bias(block.trim_offset),
            crate::f32_from_rpu_u12_with_bias(block.trim_power),
            crate::f32_from_rpu_u12_with_bias(block.trim_chroma_weight),
            crate::f32_from_rpu_u12_with_bias(block.trim_saturation_gain),
            crate::f32_from_rpu_u12_with_bias(block.ms_weight),
        ]);

        trim.sop_to_lgg();

        Self {
            level: 8,
            tid: block.target_display_index,
            l8_trim: CMV40(trim),
            mid_contrast_bias: crate::f32_from_rpu_u12_with_bias(block.target_mid_contrast),
            highlight_clipping: crate::f32_from_rpu_u12_with_bias(block.clip_trim),
            sat_vector_field: CMV40(TrimSixField([
                crate::f32_from_rpu_u8_with_bias(block.saturation_vector_field0),
                crate::f32_from_rpu_u8_with_bias(block.saturation_vector_field1),
                crate::f32_from_rpu_u8_with_bias(block.saturation_vector_field2),
                crate::f32_from_rpu_u8_with_bias(block.saturation_vector_field3),
                crate::f32_from_rpu_u8_with_bias(block.saturation_vector_field4),
                crate::f32_from_rpu_u8_with_bias(block.saturation_vector_field5),
            ])),
            hue_vector_field: CMV40(TrimSixField([
                crate::f32_from_rpu_u8_with_bias(block.hue_vector_field0),
                crate::f32_from_rpu_u8_with_bias(block.hue_vector_field1),
                crate::f32_from_rpu_u8_with_bias(block.hue_vector_field2),
                crate::f32_from_rpu_u8_with_bias(block.hue_vector_field3),
                crate::f32_from_rpu_u8_with_bias(block.hue_vector_field4),
                crate::f32_from_rpu_u8_with_bias(block.hue_vector_field5),
            ])),
        }
    }
}
