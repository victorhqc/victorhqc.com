use async_graphql::{SimpleObject, Union, ID};
use core_victorhqc_com::models::fujifilm::{
    FujifilmRecipe as FujifilmRecipeModel, Settings as SettingsModel,
};
use fuji::recipe::Settings;

#[derive(SimpleObject, Clone)]
pub struct FujifilmRecipe {
    pub id: ID,
    pub name: String,
    pub src: String,
    pub film_simulation: String,
    pub sensor: String,
    pub settings: FujifilmRecipeSettings,
}

#[derive(Union, Clone)]
pub enum FujifilmRecipeSettings {
    TransI(FujifilmRecipeTransISettings),
    TransII(FujifilmRecipeTransIISettings),
    TransIII(FujifilmRecipeTransIIISettings),
    TransIV(FujifilmRecipeTransIVSettings),
    TransV(FujifilmRecipeTransVSettings),
}

#[derive(SimpleObject, PartialEq, Clone)]
pub struct FujifilmRecipeTransISettings {
    pub white_balance: String,
    pub dynamic_range: String,
    pub tone_curve: String,
    pub color: String,
    pub sharpness: String,
    pub high_iso_noise_reduction: String,
}

#[derive(SimpleObject, PartialEq, Clone)]
pub struct FujifilmRecipeTransIISettings {
    pub white_balance: String,
    pub dynamic_range: String,
    pub tone_curve: String,
    pub color: String,
    pub sharpness: String,
    pub high_iso_noise_reduction: String,
}

#[derive(SimpleObject, PartialEq, Clone)]
pub struct FujifilmRecipeTransIIISettings {
    pub white_balance: String,
    pub dynamic_range: String,
    pub grain_effect: String,
    pub tone_curve: String,
    pub color: String,
    pub monochromatic_color: String,
    pub sharpness: String,
    pub high_iso_noise_reduction: String,
}

#[derive(SimpleObject, PartialEq, Clone)]
pub struct FujifilmRecipeTransIVSettings {
    pub white_balance: String,
    pub dynamic_range: String,
    pub d_range_priority: String,
    pub grain_effect: String,
    pub color_chrome_effect: String,
    pub color_chrome_fx_blue: String,
    pub tone_curve: String,
    pub color: String,
    pub monochromatic_color: String,
    pub sharpness: String,
    pub high_iso_noise_reduction: String,
    pub clarity: String,
}

#[derive(SimpleObject, PartialEq, Clone)]
pub struct FujifilmRecipeTransVSettings {
    pub white_balance: String,
    pub dynamic_range: String,
    pub d_range_priority: String,
    pub grain_effect: String,
    pub color_chrome_effect: String,
    pub color_chrome_fx_blue: String,
    pub tone_curve: String,
    pub color: String,
    pub monochromatic_color: String,
    pub sharpness: String,
    pub high_iso_noise_reduction: String,
    pub clarity: String,
}

impl From<FujifilmRecipeModel> for FujifilmRecipe {
    fn from(model: FujifilmRecipeModel) -> Self {
        let settings = SettingsModel(model.details().settings.clone());

        FujifilmRecipe {
            id: model.id.clone().into(),
            name: model.name.clone(),
            src: model.src.clone(),
            film_simulation: model.details().film_simulation.to_string(),
            sensor: model.details().sensor.to_string(),
            settings: settings.into(),
        }
    }
}

impl From<SettingsModel> for FujifilmRecipeSettings {
    fn from(value: SettingsModel) -> Self {
        match value.0 {
            Settings::TransI(s) => FujifilmRecipeSettings::TransI(FujifilmRecipeTransISettings {
                white_balance: s.white_balance.to_string(),
                dynamic_range: s.dynamic_range.to_string(),
                tone_curve: s.tone_curve.to_string(),
                color: s.color.to_string(),
                sharpness: s.sharpness.to_string(),
                high_iso_noise_reduction: s.high_iso_noise_reduction.to_string(),
            }),
            Settings::TransII(s) => {
                FujifilmRecipeSettings::TransII(FujifilmRecipeTransIISettings {
                    white_balance: s.white_balance.to_string(),
                    dynamic_range: s.dynamic_range.to_string(),
                    tone_curve: s.tone_curve.to_string(),
                    color: s.color.to_string(),
                    sharpness: s.sharpness.to_string(),
                    high_iso_noise_reduction: s.high_iso_noise_reduction.to_string(),
                })
            }
            Settings::TransIII(s) => {
                FujifilmRecipeSettings::TransIII(FujifilmRecipeTransIIISettings {
                    white_balance: s.white_balance.to_string(),
                    dynamic_range: s.dynamic_range.to_string(),
                    grain_effect: s.grain_effect.to_string(),
                    tone_curve: s.tone_curve.to_string(),
                    color: s.color.to_string(),
                    monochromatic_color: s.monochromatic_color.to_string(),
                    sharpness: s.sharpness.to_string(),
                    high_iso_noise_reduction: s.high_iso_noise_reduction.to_string(),
                })
            }
            Settings::TransIV(s) => {
                FujifilmRecipeSettings::TransIV(FujifilmRecipeTransIVSettings {
                    white_balance: s.white_balance.to_string(),
                    dynamic_range: s.dynamic_range.to_string(),
                    d_range_priority: s.d_range_priority.to_string(),
                    grain_effect: s.grain_effect.to_string(),
                    color_chrome_effect: s.color_chrome_effect.to_string(),
                    color_chrome_fx_blue: s.color_chrome_fx_blue.to_string(),
                    tone_curve: s.tone_curve.to_string(),
                    color: s.color.to_string(),
                    monochromatic_color: s.monochromatic_color.to_string(),
                    sharpness: s.sharpness.to_string(),
                    high_iso_noise_reduction: s.high_iso_noise_reduction.to_string(),
                    clarity: s.clarity.to_string(),
                })
            }
            Settings::TransV(s) => FujifilmRecipeSettings::TransV(FujifilmRecipeTransVSettings {
                white_balance: s.white_balance.to_string(),
                dynamic_range: s.dynamic_range.to_string(),
                d_range_priority: s.d_range_priority.to_string(),
                grain_effect: s.grain_effect.to_string(),
                color_chrome_effect: s.color_chrome_effect.to_string(),
                color_chrome_fx_blue: s.color_chrome_fx_blue.to_string(),
                tone_curve: s.tone_curve.to_string(),
                color: s.color.to_string(),
                monochromatic_color: s.monochromatic_color.to_string(),
                sharpness: s.sharpness.to_string(),
                high_iso_noise_reduction: s.high_iso_noise_reduction.to_string(),
                clarity: s.clarity.to_string(),
            }),
        }
    }
}
