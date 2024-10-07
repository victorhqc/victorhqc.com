use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::DRangePriority;

impl FromExifData for DRangePriority {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let value: String = if let Some(exif) = data.find("DRangePriorityAuto") {
            exif.value().to_string()
        } else if let Some(exif) = data.find("DRangePriority") {
            exif.value().to_string()
        } else {
            "".to_string()
        };

        match value.to_lowercase().as_str() {
            "strong" => Some(DRangePriority::Strong),
            "weak" => Some(DRangePriority::Weak),
            "auto" => Some(DRangePriority::Auto),
            _ => Some(DRangePriority::Off),
        }
    }
}
