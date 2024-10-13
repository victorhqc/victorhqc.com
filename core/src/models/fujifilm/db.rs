use crate::models::fujifilm::{
    builder::SettingsBuilder,
    from_tuple::{grain_effect::Error as GrainEffectError, FromTuple},
    str::Error as RecipeError,
    Clarity, Color, ColorChromeEffect, ColorChromeEffectFxBlue, DRangePriority, DynamicRange,
    FilmSimulation, FujifilmRecipe, FujifilmRecipeDetails, GrainEffect, GrainSize, GrainStrength,
    HighISONoiseReduction, MonochromaticColor, SettingStrength, Sharpness, ToneCurve, TransSensor,
    WBShift, WhiteBalance,
};
use snafu::prelude::*;
use sqlx::{error::Error as SqlxError, FromRow, SqliteConnection, SqlitePool};
use std::str::FromStr;
use strum_macros::Display;

#[derive(Clone, Debug, FromRow)]
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

#[derive(FromRow)]
pub struct DBExifMetaFujifilmRecipe {
    pub exif_meta_id: String,
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

    pub async fn find_by_exif_meta_ids(
        pool: &SqlitePool,
        ids: &Vec<String>,
    ) -> Result<Vec<(String, FujifilmRecipe)>, Error> {
        find_by_exif_meta_ids(pool, ids).await
    }

    pub async fn find_by_details(
        pool: &SqlitePool,
        details: &FujifilmRecipeDetails,
    ) -> Result<Option<FujifilmRecipe>, Error> {
        find_by_recipe_details(pool, details).await
    }

    pub async fn save(&self, conn: &mut SqliteConnection) -> Result<String, Error> {
        let recipe: DBFujifilmRecipe = self.into();

        insert(conn, recipe).await
    }
}

async fn find_by_film_simulation(
    pool: &SqlitePool,
    name: &str,
) -> Result<Vec<FujifilmRecipe>, Error> {
    // TODO: Move back to macro. Fails to compile in IDE because fails to find DB
    let recipes = sqlx::query_as::<_, DBFujifilmRecipe>(
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
    )
    .bind(name)
    .fetch_all(pool)
    .await
    .context(SqlxSnafu)?;

    let recipes = recipes.into_iter().map(|r| r.try_into().unwrap()).collect();

    Ok(recipes)
}

async fn find_by_exif_meta_ids(
    pool: &SqlitePool,
    ids: &Vec<String>,
) -> Result<Vec<(String, FujifilmRecipe)>, Error> {
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));

    let query = format!(
        r#"
    SELECT
        e.id exif_meta_id,
        r.id,
        r.name,
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
        fuji_recipes r
    JOIN exif_metas e ON e.fuji_recipe_id = r.id
    WHERE
        exif_meta_id IN ( { } )
    "#,
        params
    );

    let mut query = sqlx::query_as::<_, DBExifMetaFujifilmRecipe>(&query);

    for id in ids {
        query = query.bind(id);
    }

    let recipes = query.fetch_all(pool).await.context(SqlxSnafu)?;

    let recipes: Vec<(String, FujifilmRecipe)> = recipes
        .into_iter()
        .map(|r| {
            (
                r.exif_meta_id,
                DBFujifilmRecipe {
                    id: r.id,
                    name: r.name,
                    author: r.author,
                    src: r.src,
                    sensor: r.sensor,
                    film_simulation: r.film_simulation,
                    white_balance: r.white_balance,
                    white_balance_shift: r.white_balance_shift,
                    dynamic_range: r.dynamic_range,
                    d_range_priority: r.d_range_priority,
                    highlight_tone: r.highlight_tone,
                    shadow_tone: r.shadow_tone,
                    color: r.color,
                    sharpness: r.sharpness,
                    clarity: r.clarity,
                    high_iso_noise_reduction: r.high_iso_noise_reduction,
                    grain_strength: r.grain_strength,
                    grain_size: r.grain_size,
                    color_chrome_effect: r.color_chrome_effect,
                    color_chrome_fx_blue: r.color_chrome_fx_blue,
                    monochromatic_color: r.monochromatic_color,
                },
            )
        })
        .map(|(id, r)| (id, r.try_into().unwrap()))
        .collect();

    Ok(recipes)
}

async fn find_by_recipe_details(
    pool: &SqlitePool,
    details: &FujifilmRecipeDetails,
) -> Result<Option<FujifilmRecipe>, Error> {
    // TODO: Move back to macro. Fails to compile in IDE because fails to find DB
    let mut query = sqlx::query_as::<_, DBFujifilmRecipe>(
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
        AND sensor = ?
        AND white_balance =?
        AND white_balance_shift = ?
        AND dynamic_range = ?
        AND d_range_priority = ?
        AND highlight_tone = ?
        AND shadow_tone = ?
        AND color = ?
        AND sharpness = ?
        AND clarity = ?
        AND high_iso_noise_reduction = ?
        AND grain_strength = ?
        AND grain_size = ?
        AND color_chrome_effect = ?
        AND color_chrome_fx_blue = ?
        AND monochromatic_color = ?
    "#,
    );

    let (
        white_balance,
        dynamic_range,
        d_range_priority,
        tone,
        color,
        sharpness,
        clarity,
        high_iso_nr,
        grain_effect,
        color_chrome_effect,
        color_chrome_fx_blue,
        monochromatic_color,
    ) = details.settings.get_values();

    let shift = white_balance.get_shift();

    query = query
        .bind(details.film_simulation.to_string())
        .bind(details.sensor.to_string())
        .bind(white_balance.to_string_no_shift())
        .bind(shift.to_string())
        .bind(dynamic_range.to_string())
        .bind(d_range_priority.map(|d| d.to_string()))
        .bind(tone.highlights)
        .bind(tone.shadows)
        .bind(color.value)
        .bind(sharpness.value)
        .bind(clarity.map(|c| c.value))
        .bind(high_iso_nr.value)
        .bind(grain_effect.as_ref().map(|g| g.grain_strength_to_string()))
        .bind(grain_effect.as_ref().map(|g| g.grain_size_to_string()))
        .bind(color_chrome_effect.map(|c| c.to_string()))
        .bind(color_chrome_fx_blue.map(|c| c.to_string()))
        .bind(monochromatic_color.map(|c| c.to_string()));

    let recipe = query.fetch_optional(pool).await.context(SqlxSnafu)?;

    let recipe = if let Some(r) = recipe.map(|r| r.try_into()) {
        Some(r?)
    } else {
        None
    };

    Ok(recipe)
}

async fn insert(conn: &mut SqliteConnection, recipe: DBFujifilmRecipe) -> Result<String, Error> {
    let id = recipe.id.clone();

    sqlx::query(
        r#"
    INSERT INTO fuji_recipes(
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
    )
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#,
    )
    .bind(recipe.id)
    .bind(recipe.name)
    .bind(recipe.author)
    .bind(recipe.sensor)
    .bind(recipe.src)
    .bind(recipe.film_simulation)
    .bind(recipe.white_balance)
    .bind(recipe.white_balance_shift)
    .bind(recipe.dynamic_range)
    .bind(recipe.d_range_priority)
    .bind(recipe.highlight_tone)
    .bind(recipe.shadow_tone)
    .bind(recipe.color)
    .bind(recipe.sharpness)
    .bind(recipe.clarity)
    .bind(recipe.high_iso_noise_reduction)
    .bind(recipe.grain_strength)
    .bind(recipe.grain_size)
    .bind(recipe.color_chrome_effect)
    .bind(recipe.color_chrome_fx_blue)
    .bind(recipe.monochromatic_color)
    .execute(conn)
    .await
    .context(SqlxSnafu)?;

    Ok(id)
}

impl TryFrom<DBFujifilmRecipe> for FujifilmRecipe {
    type Error = Error;

    fn try_from(value: DBFujifilmRecipe) -> Result<Self, Self::Error> {
        let mut builder = SettingsBuilder::default();

        let film_simulation =
            FilmSimulation::from_str(&value.film_simulation).context(RecipeSnafu)?;

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

        let sharpness = Sharpness {
            value: value.sharpness,
        };

        let high_iso_reduction = HighISONoiseReduction {
            value: value.high_iso_noise_reduction,
        };

        let clarity = value.clarity.map(|c| Clarity { value: c });

        builder
            .with_white_balance(Some(white_balance))
            .with_dynamic_range(Some(dynamic_range))
            .with_d_range_priority(d_range_priority)
            .with_grain_effect(Some(grain_effect))
            .with_color_chrome_effect(color_chrome_effect)
            .with_color_chrome_fx_blue(color_chrome_fx_blue)
            .with_tone_curve(Some(tone_curve))
            .with_color(Some(color))
            .with_monochromatic_color(monochromatic_color)
            .with_sharpness(Some(sharpness))
            .with_high_iso_noise_reduction(Some(high_iso_reduction))
            .with_clarity(clarity);

        let settings = trans_sensor.settings(builder);

        Ok(FujifilmRecipe {
            id: value.id,
            name: value.name,
            author: value.author,
            src: value.src,
            details: FujifilmRecipeDetails {
                sensor: trans_sensor,
                film_simulation,
                settings,
            },
        })
    }
}

impl From<&FujifilmRecipe> for DBFujifilmRecipe {
    fn from(value: &FujifilmRecipe) -> Self {
        let (
            white_balance,
            dynamic_range,
            d_range_priority,
            tone,
            color,
            sharpness,
            clarity,
            high_iso_nr,
            grain_effect,
            color_chrome_effect,
            color_chrome_fx_blue,
            monochromatic_color,
        ) = value.details.settings.get_values();

        DBFujifilmRecipe {
            id: value.id.clone(),
            name: value.name.clone(),
            author: value.author.clone(),
            src: value.src.clone(),
            sensor: value.details.sensor.to_string(),
            film_simulation: value.details.film_simulation.to_string(),
            white_balance: white_balance.to_string_no_shift(),
            white_balance_shift: white_balance.get_shift().to_string(),
            dynamic_range: dynamic_range.to_string(),
            d_range_priority: d_range_priority.map(|d| d.to_string()),
            highlight_tone: tone.highlights,
            shadow_tone: tone.shadows,
            color: color.value,
            sharpness: sharpness.value,
            clarity: clarity.map(|c| c.value),
            high_iso_noise_reduction: high_iso_nr.value,
            grain_strength: grain_effect
                .as_ref()
                .and_then(|g| g.grain_strength_to_string()),
            grain_size: grain_effect.as_ref().and_then(|g| g.grain_size_to_string()),
            color_chrome_effect: color_chrome_effect.map(|c| c.to_string()),
            color_chrome_fx_blue: color_chrome_fx_blue.map(|c| c.to_string()),
            monochromatic_color: monochromatic_color.map(|m| m.to_string()),
        }
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
    TransSensor,
    DynamicRange,
    GrainStrength,
    ColorChromeEffect,
    ColorChromeFxBlue,
}
