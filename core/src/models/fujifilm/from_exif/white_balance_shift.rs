use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::WBShift;
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for WBShift {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("WhiteBalanceFineTune")?;

        debug!("WBShift::from_exif: {:?}", exif);

        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i:(red ?[+\-0-9]+), ?(blue ?[+\-0-9]+))").unwrap());

        let captures = RE.captures(exif.value())?;

        debug!("WBShift Captures: {:?}", captures);

        let red = captures.get(1)?;
        let blue = captures.get(2)?;

        let red = red.as_str().to_lowercase().replace("red ", "");
        let blue = blue.as_str().to_lowercase().replace("blue ", "");

        let red: i32 = if let Ok(r) = red.parse::<i32>() {
            r
        } else {
            return None;
        };

        let blue: i32 = if let Ok(b) = blue.parse::<i32>() {
            b
        } else {
            return None;
        };

        let red = red / 10 / 2;
        let blue = blue / 10 / 2;

        debug!("red {:?}", red);
        debug!("blue {:?}", blue);

        Some(WBShift { red, blue })
    }
}
