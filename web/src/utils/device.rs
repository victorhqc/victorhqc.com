use std::str::FromStr;

use strum_macros::{Display, EnumString};
use uaparser::{Device, OS};

#[derive(Debug, Clone, Display, PartialEq, EnumString, serde::Serialize)]
pub enum KnownOS {
    #[strum(serialize = "iOS")]
    #[serde(rename(serialize = "iOS"))]
    Ios,
    Android,
    #[strum(serialize = "KaiOS")]
    #[serde(rename(serialize = "KaiOS"))]
    Kaios,
}

#[derive(Debug, Clone, Display, PartialEq, EnumString, serde::Serialize)]
pub enum KnownDevice {
    #[strum(serialize = "iPad")]
    #[serde(rename(serialize = "iPad"))]
    IPad,
}

pub fn is_mobile(device: &Device, os: &OS) -> bool {
    let known_os = KnownOS::from_str(&os.family).ok();
    let known_device = KnownDevice::from_str(&device.family).ok();

    // Mobile if it's iOS, Android or KaiOS and it's NOT an iPad
    matches!((known_os, known_device), (Some(_), None))
}
