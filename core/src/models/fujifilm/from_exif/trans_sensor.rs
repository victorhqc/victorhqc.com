use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::TransSensor;
use log::debug;

impl FromExifData for TransSensor {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Model")?;

        debug!("TransSensor::from_exif: {:?}", exif);

        match exif.value().to_lowercase().as_str() {
            "x-pro1" => Some(TransSensor::TransI),
            "x-e1" => Some(TransSensor::TransI),
            "x-m1" => Some(TransSensor::TransI),
            "x100s" => Some(TransSensor::TransII),
            "x-e2" => Some(TransSensor::TransII),
            "x-t1" => Some(TransSensor::TransII),
            "x100t" => Some(TransSensor::TransII),
            "x-t10" => Some(TransSensor::TransII),
            "x-e2s" => Some(TransSensor::TransII),
            "x70" => Some(TransSensor::TransII),
            "x20" => Some(TransSensor::TransII),
            "xq1" => Some(TransSensor::TransII),
            "x30" => Some(TransSensor::TransII),
            "xq2" => Some(TransSensor::TransII),
            "x-pro2" => Some(TransSensor::TransIII),
            "x-t2" => Some(TransSensor::TransIII),
            "x100f" => Some(TransSensor::TransIII),
            "x-t20" => Some(TransSensor::TransIII),
            "x-e3" => Some(TransSensor::TransIII),
            "x-h1" => Some(TransSensor::TransIII),
            "x-t3" => Some(TransSensor::TransIV),
            "x-t4" => Some(TransSensor::TransIV),
            "x-t30" => Some(TransSensor::TransIV),
            "x-t30 ii" => Some(TransSensor::TransIV),
            "x-pro3" => Some(TransSensor::TransIV),
            "x100v" => Some(TransSensor::TransIV),
            "x-s10" => Some(TransSensor::TransIV),
            "x-s20" => Some(TransSensor::TransIV),
            "x-e4" => Some(TransSensor::TransIV),
            "x-h2s" => Some(TransSensor::TransV),
            "x-h2" => Some(TransSensor::TransV),
            "x-t5" => Some(TransSensor::TransV),
            "x-t50" => Some(TransSensor::TransV),
            "x100vi" => Some(TransSensor::TransV),
            _ => None,
        }
    }
}
