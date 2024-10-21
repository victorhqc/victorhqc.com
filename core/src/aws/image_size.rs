use snafu::Snafu;
use std::str::FromStr;
use strum_macros::Display;

#[derive(Debug, Display)]
pub enum ImageSize {
    Hd,
    Md,
    Sm,
}

impl FromStr for ImageSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Hd" => Ok(ImageSize::Hd),
            "hd" => Ok(ImageSize::Hd),
            "Md" => Ok(ImageSize::Md),
            "md" => Ok(ImageSize::Md),
            "Sm" => Ok(ImageSize::Sm),
            "sm" => Ok(ImageSize::Sm),
            _ => Err(Error::Invalid),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid string"))]
    Invalid,
}
