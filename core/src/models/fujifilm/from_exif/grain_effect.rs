use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::{GrainEffect, GrainSize, GrainStrength};
use log::debug;

impl FromExifData for GrainEffect {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let strength_exif = data.find("GrainEffectRoughness")?;

        debug!("GrainStrength::from_exif: {:?}", strength_exif);

        let size_exif = data.find("GrainEffectSize").unwrap_or_default();
        
        debug!("GrainSize::from_exif: {:?}", size_exif);

        let grain_size = match size_exif.value().to_lowercase().as_str() {
            "small" => Some(GrainSize::Small),
            "large" => Some(GrainSize::Large),
            _ => None,
        };


        if let Some(grain_size) = grain_size {
            match strength_exif.value().to_lowercase().as_str() {
                "strong" => Some(GrainEffect::StrengthAndSize {
                    strength: GrainStrength::Strong,
                    size: grain_size,
                }),
                "weak" => Some(GrainEffect::StrengthAndSize {
                    strength: GrainStrength::Weak,
                    size: grain_size,
                }),
                _ => Some(GrainEffect::Off),
            }
        } else {
            match strength_exif.value().to_lowercase().as_str() {
                "strong" => Some(GrainEffect::OnlyStrength {
                    strength: GrainStrength::Strong,
                }),
                "weak" => Some(GrainEffect::OnlyStrength {
                    strength: GrainStrength::Weak,
                }),
                _ => Some(GrainEffect::Off),
            }
        }
    }
}
