use log::trace;
use snafu::prelude::*;

pub struct JsonValue(pub serde_json::Value);

impl TryFrom<JsonValue> for Vec<ExifData> {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let mut fields: Vec<ExifData> = value.0
            .as_object()
            .context(EmptySnafu)?
            .into_iter()
            .filter_map(|(tag, value)| {
                trace!("{:?}: {:?}", tag, value);

                if let Some(value) = value.as_str() {
                    let value = value.to_string().trim().to_string();

                    Some((tag.clone(), value))
                } else if let Some(value) = value.as_number() {
                    Some((tag.clone(), value.to_string()))
                } else if let Some(values) = value.as_array() {
                    let value: String = values.iter().fold(String::from(""), |acc, value| {
                        let value: String = if let Some(value) = value.as_str() {
                            let value = value.to_string().trim().to_string();

                            format!("{}, {}", acc,value)
                        } else {
                            acc
                        };

                        value
                    });

                    Some((tag.clone(), value))
                } else {
                    None
                }
            })
            .collect();

        fields.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        Ok(fields)
    }
}

pub type Tag = String;
pub type Value = String;
pub type ExifData = (Tag, Value);

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("No Object found"))]
    Empty,
}
