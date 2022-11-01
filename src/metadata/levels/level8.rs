use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel8;
use serde::Serialize;

use crate::MDFType;
use crate::MDFType::CMV40;

use super::TrimSixField;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Level8 {
    pub level: u8,
    #[serde(rename = "$unflatten=TID")]
    pub tid: u8,
    // Format: f32 f32 f32 f32 f32 f32
    #[serde(rename = "$unflatten=L8Trim")]
    pub l8_trim: MDFType<TrimSixField>,
    #[serde(rename = "$unflatten=MidContrastBias")]
    pub mid_contrast_bias: f32,
    #[serde(rename = "$unflatten=HighlightClipping")]
    pub highlight_clipping: f32,
    // Format: f32 f32 f32 f32 f32 f32
    #[serde(rename = "$unflatten=SaturationVectorField")]
    pub sat_vector_field: MDFType<TrimSixField>,
    // Format: f32 f32 f32 f32 f32 f32
    #[serde(rename = "$unflatten=HueVectorField")]
    pub hue_vector_field: MDFType<TrimSixField>,
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
