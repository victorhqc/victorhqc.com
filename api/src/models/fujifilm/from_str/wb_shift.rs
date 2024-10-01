use crate::models::fujifilm::{
    from_str::{Error, ParseKey},
    WBShift,
};
use rocket::http::ext::IntoOwned;
use std::str::FromStr;

impl FromStr for WBShift {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shifts: Vec<i32> = String::from(s)
            .split(",")
            .map(|val| val.parse::<i32>())
            .filter_map(|e| e.ok())
            .collect();

        if shifts.len() != 2 {
            return Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: format!(
                    "Wrong amount of values, should be 2 but got: {}",
                    shifts.len(),
                ),
            });
        }

        let red: &i32 = shifts.first().unwrap_or(&0);
        let blue: &i32 = shifts.get(1).unwrap_or(&0);

        Ok(WBShift {
            red: red.into_owned(),
            blue: blue.into_owned(),
        })
    }
}
