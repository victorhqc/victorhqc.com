use crate::models::fujifilm::from_tuple::FromTuple;
use fuji::recipe::{GrainEffect, GrainSize, GrainStrength};
use snafu::prelude::*;

impl FromTuple<Option<GrainStrength>, Option<GrainSize>> for GrainEffect {
    type Err = Error;
    fn from_tuple(tuple: (Option<GrainStrength>, Option<GrainSize>)) -> Result<Self, Self::Err> {
        if tuple.0.is_none() && tuple.1.is_none() {
            return Ok(GrainEffect::Off);
        }

        if tuple.1.is_none() {
            if let Some(s) = tuple.0 {
                return Ok(GrainEffect::OnlyStrength { strength: s });
            }
        }

        if tuple.0.is_some() && tuple.1.is_some() {
            let strength = tuple.0.unwrap();
            let size = tuple.1.unwrap();

            return Ok(GrainEffect::StrengthAndSize { strength, size });
        }

        Err(Error::Impossible)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Tuple has wrong data"))]
    Impossible,
}
