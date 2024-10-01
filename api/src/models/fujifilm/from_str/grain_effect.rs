use crate::models::fujifilm::{
    from_str::{Error, ParseKey},
    GrainEffect,
};
use std::str::FromStr;

impl FromStr for GrainEffect {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        todo!();
    }
}
