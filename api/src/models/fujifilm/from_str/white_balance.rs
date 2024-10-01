use crate::models::fujifilm::{
    from_str::{Error, ParseKey},
    WBShift, WhiteBalance,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

impl FromStr for WhiteBalance {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-9]+)K$").unwrap());
        let caps = RE.captures(s);

        if let Some(cap) = caps {
            let temp = cap.get(1).unwrap();
            let temp: i32 = temp.as_str().to_string().parse::<i32>().unwrap();

            return Ok(WhiteBalance::Kelvin {
                temperature: temp,
                shift: WBShift::default(),
            });
        };

        match s {
            "Auto" => Ok(WhiteBalance::Auto {
                shift: WBShift::default(),
            }),
            "AutoWhitePriority" => Ok(WhiteBalance::AutoWhitePriority {
                shift: WBShift::default(),
            }),
            "Custom1" => Ok(WhiteBalance::Custom1 {
                shift: WBShift::default(),
            }),
            "Custom2" => Ok(WhiteBalance::Custom2 {
                shift: WBShift::default(),
            }),
            "Custom3" => Ok(WhiteBalance::Custom3 {
                shift: WBShift::default(),
            }),
            "Daylight" => Ok(WhiteBalance::Daylight {
                shift: WBShift::default(),
            }),
            "Cloudy" => Ok(WhiteBalance::Cloudy {
                shift: WBShift::default(),
            }),
            "FluorescentLight1" => Ok(WhiteBalance::FluorescentLight1 {
                shift: WBShift::default(),
            }),
            "FluorescentLight2" => Ok(WhiteBalance::FluorescentLight2 {
                shift: WBShift::default(),
            }),
            "FluorescentLight3" => Ok(WhiteBalance::FluorescentLight3 {
                shift: WBShift::default(),
            }),
            "Incandescent" => Ok(WhiteBalance::Incandescent {
                shift: WBShift::default(),
            }),
            "Underwater" => Ok(WhiteBalance::Underwater {
                shift: WBShift::default(),
            }),

            _ => Err(Error::Parse {
                key: ParseKey::WhiteBalance,
                reason: format!("Could not match WB to {}", s),
            }),
        }
    }
}
