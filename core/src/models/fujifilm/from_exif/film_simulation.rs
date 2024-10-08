use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::models::fujifilm::{FilmSimulation, MonochromaticFilter};
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for FilmSimulation {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = if let Some(v) = data.find("FilmMode") {
            v
        } else if let Some(v) = data.find("Saturation") {
            v
        } else {
            return None;
        };

        debug!("FilmSimulation::from_exif: {:?}", exif);

        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
            r"(?i:provia|velvia|astia|classic chrome|classic neg|nostalgic neg|acros yellow filter|acros red filter|acros green filter|acros|eterna|b&w sepia|b&w yellow filter|b&w red filter|b&w green filter|b&w|reala ace|pro neg\.? std|pro neg\.? standard|pro neg\.? hi|bleach bypass)",
        ).unwrap()
        });

        let captures = RE.captures(exif.value())?;
        
        debug!("FilmSimulation Captures: {:?}", captures);

        match captures[0].to_lowercase().as_str() {
            "provia" => Some(FilmSimulation::ProviaStandard),
            "velvia" => Some(FilmSimulation::VelviaVivid),
            "astia" => Some(FilmSimulation::AstiaSoft),
            "classic chrome" => Some(FilmSimulation::ClassicChrome),
            "classic neg" => Some(FilmSimulation::ClassicNeg),
            "nostalgic neg" => Some(FilmSimulation::NostalgicNeg),
            "pro neg. std" => Some(FilmSimulation::ProNegStd),
            "pro neg std" => Some(FilmSimulation::ProNegStd),
            "pro neg. standard" => Some(FilmSimulation::ProNegStd),
            "pro neg standard" => Some(FilmSimulation::ProNegStd),
            "pro neg. hi" => Some(FilmSimulation::ProNegHi),
            "pro neg hi" => Some(FilmSimulation::ProNegHi),
            "eterna" => Some(FilmSimulation::EternaCinema),
            "bleach bypass" => Some(FilmSimulation::BleachBypass),
            "acros yellow filter" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow,
            }),
            "acros red filter" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red,
            }),
            "acros green filter" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green,
            }),
            "acros" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Std,
            }),
            "b&w" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Std,
            }),
            "b&w yellow filter" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow,
            }),
            "b&w red filter" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red,
            }),
            "b&w green filter" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green,
            }),
            "b&w sepia" => Some(FilmSimulation::Sepia),
            "reala ace" => Some(FilmSimulation::RealaAce),
            _ => None,
        }
    }
}
