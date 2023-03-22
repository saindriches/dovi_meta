use dolby_vision::rpu::extension_metadata::blocks::ExtMetadataBlockLevel5;
use serde::Serialize;
use std::cmp::Ordering;

use crate::MDFType::CMV40;
use crate::{IntoCMV29, MDFType, UHD_HEIGHT, UHD_WIDTH};

use super::AspectRatio;

#[derive(Debug, Clone, Serialize, Hash, PartialEq, Eq)]
pub struct Level5 {
    #[serde(rename = "@level")]
    pub level: u8,
    // Format: f32 f32
    #[serde(rename = "AspectRatio")]
    pub aspect_ratio: MDFType<AspectRatio>,
}

impl Level5 {
    pub fn get_ar(&self) -> (f32, f32) {
        let ar = self.aspect_ratio.into_inner().0;
        (ar[0], ar[1])
    }
}

// For convenience, it assumes the canvas is standard UHD
impl From<&ExtMetadataBlockLevel5> for Level5 {
    fn from(block: &ExtMetadataBlockLevel5) -> Self {
        Self::with_canvas(block, (UHD_WIDTH, UHD_HEIGHT))
    }
}

impl From<f32> for Level5 {
    fn from(ar: f32) -> Self {
        Self {
            level: 5,
            aspect_ratio: CMV40(AspectRatio([ar, ar])),
        }
    }
}

impl IntoCMV29<Self> for Level5 {
    fn into_cmv29(self) -> Self {
        Self {
            level: 5,
            aspect_ratio: self.aspect_ratio.into_cmv29(),
        }
    }
}

impl Level5 {
    pub fn with_canvas(block: &ExtMetadataBlockLevel5, canvas: (usize, usize)) -> Self {
        let (width, height) = canvas;
        let canvas_ar = width as f32 / height as f32;

        let horizontal_crop = block.active_area_left_offset + block.active_area_right_offset;
        let vertical_crop = block.active_area_top_offset + block.active_area_bottom_offset;

        let image_ar = if horizontal_crop > 0 {
            (width as f32 - horizontal_crop as f32) / height as f32
        } else {
            // Ok because only one of the crop types will be 0
            width as f32 / (height as f32 - vertical_crop as f32)
        };

        Self {
            level: 5,
            aspect_ratio: CMV40(AspectRatio([canvas_ar, image_ar])),
        }
    }
}

impl PartialOrd<Self> for Level5 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.aspect_ratio
            .into_inner()
            .partial_cmp(&other.aspect_ratio.into_inner())
    }
}

impl Ord for Level5 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.aspect_ratio
            .into_inner()
            .cmp(&other.aspect_ratio.into_inner())
    }
}
