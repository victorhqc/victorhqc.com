use snafu::Snafu;
use strum_macros::Display;

mod grain_effect;
mod tone_curve;
mod wb_shift;
mod white_balance;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Failed to Parse {}: {}", key, reason))]
    Parse { key: ParseKey, reason: String },
}

#[derive(Debug, Display, PartialEq)]
pub enum ParseKey {
    WhiteBalance,
    WhiteBalanceShift,
    GrainEffect,
}
