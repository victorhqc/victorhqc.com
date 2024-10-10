use crate::models::exif_meta::Maker;
use snafu::prelude::*;
use std::str::FromStr;

impl FromStr for Maker {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fujifilm" => Ok(Maker::Fujifilm),
            "konica" => Ok(Maker::Konica),
            "canon" => Ok(Maker::Canon),
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
        assert_eq!(Maker::from_str("Fujifilm"), Ok(Maker::Fujifilm));
    }

    #[test]
    fn it_parses_konica() {
        assert_eq!(Maker::from_str("Konica"), Ok(Maker::Konica));
    }

    #[test]
    fn it_parses_canon() {
        assert_eq!(Maker::from_str("Canon"), Ok(Maker::Canon));
    }
}
