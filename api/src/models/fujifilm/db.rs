use crate::models::fujifilm::{
    builder::SettingsBuilder,
    from_tuple::{grain_effect::Error as GrainEffectError, FromTuple},
    str::Error as RecipeError,
    Color, ColorChromeEffect, ColorChromeEffectFxBlue, DRangePriority, DynamicRange,
    FilmSimulation, FujifilmRecipe, GrainEffect, GrainSize, GrainStrength, MonochromaticColor,
    SettingStrength, ToneCurve, TransSensor, WBShift, WhiteBalance,
};
use snafu::prelude::*;
use sqlx::{error::Error as SqlxError, SqlitePool};
use std::str::FromStr;
use strum_macros::Display;

#[derive(Clone, Debug)]
pub struct DBFujifilmRecipe {
    pub id: String,
    pub name: String,
    pub author: String,
    pub src: String,
    pub sensor: String,
    pub film_simulation: String,
    pub white_balance: String,
    pub white_balance_shift: String,
    pub dynamic_range: String,
    pub d_range_priority: Option<String>,
    pub highlight_tone: f64,
    pub shadow_tone: f64,
    pub color: i64,
    pub sharpness: i64,
    pub clarity: Option<i64>,
    pub high_iso_noise_reduction: i64,
    pub grain_strength: Option<String>,
    pub grain_size: Option<String>,
    pub color_chrome_effect: Option<String>,
    pub color_chrome_fx_blue: Option<String>,
    pub monochromatic_color: Option<String>,
}

impl FujifilmRecipe {
    pub async fn find_by_film_simulation(
        pool: &SqlitePool,
        name: &str,
    ) -> Result<Vec<FujifilmRecipe>, Error> {
        find_by_film_simulation(pool, name).await
    }
}

async fn find_by_film_simulation(
    pool: &SqlitePool,
    name: &str,
) -> Result<Vec<FujifilmRecipe>, Error> {
    let recipes = sqlx::query_as!(
        DBFujifilmRecipe,
        r#"
    SELECT
        id,
        name,
        author,
        sensor,
        src,
        film_simulation,
        white_balance,
        white_balance_shift,
        dynamic_range,
        d_range_priority,
        highlight_tone,
        shadow_tone,
        color,
        sharpness,
        clarity,
        high_iso_noise_reduction,
        grain_strength,
        grain_size,
        color_chrome_effect,
        color_chrome_fx_blue,
        monochromatic_color
    FROM
        fuji_recipes
    WHERE
        film_simulation = ?
    "#,
        name
    )
    .fetch_all(pool)
    .await
    .context(SqlxSnafu)?;

    let recipes = recipes.into_iter().map(|r| r.try_into().unwrap()).collect();

    Ok(recipes)
}

impl TryFrom<DBFujifilmRecipe> for FujifilmRecipe {
    type Error = Error;

    fn try_from(value: DBFujifilmRecipe) -> Result<Self, Self::Error> {
        let mut builder = SettingsBuilder::default();

        let film_simulation =
            FilmSimulation::from_str(&value.film_simulation).context(ParseSnafu {
                key: ParseProperty::FilmSimulation,
                value: value.film_simulation,
            })?;

        let trans_sensor = TransSensor::from_str(&value.sensor).context(ParseSnafu {
            key: ParseProperty::TransSensor,
            value: value.sensor,
        })?;

        let mut white_balance =
            WhiteBalance::from_str(&value.white_balance).context(RecipeSnafu)?;

        let wb_shift = WBShift::from_str(&value.white_balance_shift).context(RecipeSnafu)?;
        white_balance.set_shift(wb_shift);
        let dynamic_range = DynamicRange::from_str(&value.dynamic_range).context(ParseSnafu {
            key: ParseProperty::DynamicRange,
            value: value.dynamic_range,
        })?;

        let d_range_priority = if let Some(d) = &value.d_range_priority {
            Some(DRangePriority::from_str(d).context(ParseSnafu {
                key: ParseProperty::DynamicRange,
                value: String::from(d),
            })?)
        } else {
            None
        };

        let grain_strength = if let Some(s) = &value.grain_strength {
            Some(GrainStrength::from_str(s).context(ParseSnafu {
                key: ParseProperty::GrainStrength,
                value: String::from(s),
            })?)
        } else {
            None
        };

        let grain_size = if let Some(s) = &value.grain_size {
            Some(GrainSize::from_str(s).context(ParseSnafu {
                key: ParseProperty::GrainStrength,
                value: String::from(s),
            })?)
        } else {
            None
        };

        let grain_effect =
            GrainEffect::from_tuple((grain_strength, grain_size)).context(GrainEffectSnafu)?;

        let color_chrome_effect = if let Some(s) = &value.color_chrome_effect {
            let strength = SettingStrength::from_str(s).context(ParseSnafu {
                key: ParseProperty::ColorChromeEffect,
                value: String::from(s),
            })?;

            Some(ColorChromeEffect { strength })
        } else {
            None
        };

        let color_chrome_fx_blue = if let Some(s) = &value.color_chrome_fx_blue {
            let strength = SettingStrength::from_str(s).context(ParseSnafu {
                key: ParseProperty::ColorChromeFxBlue,
                value: String::from(s),
            })?;

            Some(ColorChromeEffectFxBlue { strength })
        } else {
            None
        };

        let tone_curve = ToneCurve {
            shadows: value.shadow_tone,
            highlights: value.highlight_tone,
        };

        let color = Color { value: value.color };

        let monochromatic_color = if let Some(mc) = value.monochromatic_color {
            Some(MonochromaticColor::from_str(&mc).context(RecipeSnafu)?)
        } else {
            None
        };

        builder
            .with_white_balance(Some(white_balance))
            .with_dynamic_range(Some(dynamic_range))
            .with_d_range_priority(d_range_priority)
            .with_grain_effect(Some(grain_effect))
            .with_color_chrome_effect(color_chrome_effect)
            .with_color_chrome_fx_blue(color_chrome_fx_blue)
            .with_tone_curve(Some(tone_curve))
            .with_color(Some(color))
            .with_monochromatic_color(monochromatic_color);

        todo!()
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to execute query: {:?}", source))]
    Sqlx { source: SqlxError },

    #[snafu(display("Failed to parse Enum value {}: {:?}", value, source))]
    Parse {
        source: strum::ParseError,
        key: ParseProperty,
        value: String,
    },

    #[snafu(display("{:?}", source))]
    Recipe { source: RecipeError },

    #[snafu(display("{:?}", source))]
    GrainEffect { source: GrainEffectError },
}

#[derive(Debug, Display)]
pub enum ParseProperty {
    FilmSimulation,
    TransSensor,
    DynamicRange,
    DRangePriority,
    GrainStrength,
    GrainSize,
    ColorChromeEffect,
    ColorChromeFxBlue,
}
