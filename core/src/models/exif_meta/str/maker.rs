use crate::models::exif_meta::{CameraMaker, LensMaker};
use snafu::prelude::*;
use std::str::FromStr;

impl FromStr for CameraMaker {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fujifilm" => Ok(Self::Fujifilm),
            "konica" => Ok(Self::Konica),
            "canon" => Ok(Self::Canon),
            _ => Err(Error::NotValid {
                name: s.to_string(),
            }),
        }
    }
}

impl FromStr for LensMaker {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fujifilm" => Ok(Self::Fujifilm),
            "konica" => Ok(Self::Konica),
            "canon" => Ok(Self::Canon),
            "7artisans" => Ok(Self::SevenArtisans),
            "unknown" => Ok(Self::Unknown),
            _ => Err(Error::NotValid {
                name: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid Maker: {}", name))]
    NotValid { name: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_fujifilm() {
        assert_eq!(CameraMaker::from_str("Fujifilm"), Ok(CameraMaker::Fujifilm));
        assert_eq!(CameraMaker::from_str("Fujifilm"), Ok(CameraMaker::Fujifilm));
    }

    #[test]
    fn it_parses_konica() {
        assert_eq!(CameraMaker::from_str("Konica"), Ok(CameraMaker::Konica));
        assert_eq!(LensMaker::from_str("Konica"), Ok(LensMaker::Konica));
    }

    #[test]
    fn it_parses_canon() {
        assert_eq!(CameraMaker::from_str("Canon"), Ok(CameraMaker::Canon));
        assert_eq!(LensMaker::from_str("Canon"), Ok(LensMaker::Canon));
    }

    #[test]
    fn it_parses_7artisans() {
        assert_eq!(
            LensMaker::from_str("7Artisans"),
            Ok(LensMaker::SevenArtisans)
        );
    }

    #[test]
    fn it_parses_unknown() {
        assert_eq!(LensMaker::from_str("Unknown"), Ok(LensMaker::Unknown));
    }
}
