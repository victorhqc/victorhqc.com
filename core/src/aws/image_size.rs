use snafu::Snafu;
use std::str::FromStr;
use strum_macros::Display;

#[derive(Debug, Display, Clone, PartialEq)]
pub enum ImageSize {
    HdPlus,
    Hd,
    Md,
    Sm,
    Blur,
}

impl FromStr for ImageSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hdplus" => Ok(ImageSize::HdPlus),
            "hd" => Ok(ImageSize::Hd),
            "md" => Ok(ImageSize::Md),
            "sm" => Ok(ImageSize::Sm),
            "blur" => Ok(ImageSize::Blur),
            _ => Err(Error::Invalid),
        }
    }
}

#[derive(Debug, Display, Clone, PartialEq)]
pub enum ImageType {
    Jpeg,
    Webp,
}

impl FromStr for ImageType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "jpeg" => Ok(ImageType::Jpeg),
            "jpg" => Ok(ImageType::Jpeg),
            "webp" => Ok(ImageType::Webp),
            _ => Err(Error::Invalid),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid string"))]
    Invalid,
}
