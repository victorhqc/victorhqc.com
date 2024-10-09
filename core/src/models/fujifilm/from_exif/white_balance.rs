use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::{WBShift, WhiteBalance};
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for WhiteBalance {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let shift = WBShift::from_exif(data).unwrap_or_default();
        let exif = data.find("WhiteBalance")?;

        debug!("WhiteBalance::from_exif: {:?}", exif);

        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?i:^(auto \(white priority\)|auto \(ambiance priority\)|auto|custom|custom2|custom3|kelvin|daylight fluorescent|day white fluorescent|white fluorescent|daylight|cloudy|incandescent|underwater)$)",
            ).unwrap()
        });

        let captures = RE.captures(exif.value())?;

        debug!("WhiteBalance Captures: {:?}", captures);

        match captures[1].to_lowercase().as_str() {
            "auto" => Some(WhiteBalance::Auto { shift }),
            "auto (white priority)" => Some(WhiteBalance::AutoWhitePriority { shift }),
            "auto (ambiance priority)" => Some(WhiteBalance::AutoAmbiencePriority { shift }),
            "custom" => Some(WhiteBalance::Custom1 { shift }),
            "custom2" => Some(WhiteBalance::Custom2 { shift }),
            "custom3" => Some(WhiteBalance::Custom3 { shift }),
            "daylight" => Some(WhiteBalance::Daylight { shift }),
            "cloudy" => Some(WhiteBalance::Cloudy { shift }),
            "daylight fluorescent" => Some(WhiteBalance::FluorescentLight1 { shift }),
            "day white fluorescent" => Some(WhiteBalance::FluorescentLight2 { shift }),
            "white fluorescent" => Some(WhiteBalance::FluorescentLight3 { shift }),
            "incandescent" => Some(WhiteBalance::Incandescent { shift }),
            "underwater" => Some(WhiteBalance::Underwater { shift }),
            "kelvin" => {
                if let Some(exif) = data.find("Color Temperature") {
                    debug!("WhiteBalance::Temperature {:?}", exif.value());

                    if let Ok(temperature) = exif.value().parse::<i32>() {
                        Some(WhiteBalance::Kelvin { temperature, shift })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_white_balance_auto() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Auto")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Auto {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Auto"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Auto {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }
}
