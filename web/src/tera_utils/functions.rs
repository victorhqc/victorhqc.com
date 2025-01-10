use core_victorhqc_com::models::fujifilm::{FilmSimulation, MonochromaticFilter};
use std::{collections::HashMap, str::FromStr};
use tera::{from_value, to_value, Function, Result, Value};

pub fn get_film_simulation_image() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        let name = args.get("name").unwrap();
        let name = from_value::<String>(name.clone()).unwrap();
        let film_sim = FilmSimulation::from_str(&name).expect("Invalid film sim");

        let img = match film_sim {
            FilmSimulation::ProviaStandard => "provia_1.png",
            FilmSimulation::RealaAce => "realaace_1.png",
            FilmSimulation::VelviaVivid => "velvia_1.png",
            FilmSimulation::AstiaSoft => "astia_1.png",
            FilmSimulation::ClassicChrome => "classicchrome_1.png",
            FilmSimulation::ClassicNeg => "classicneg_1.png",
            FilmSimulation::ProNegHi => "proneghi_1.png",
            FilmSimulation::ProNegStd => "pronegstd_1.png",
            FilmSimulation::NostalgicNeg => "nostalgicneg_1.png",
            FilmSimulation::EternaCinema => "eterna_1.png",
            FilmSimulation::BleachBypass => "eternabb_1.png",
            FilmSimulation::Sepia => "sepia_1.png",
            FilmSimulation::Monochrome { filter } => match filter {
                MonochromaticFilter::Std => "monostd_1.png",
                MonochromaticFilter::Red => "monor_1.png",
                MonochromaticFilter::Green => "monog_1.png",
                MonochromaticFilter::Yellow => "monoye_1.png",
            },
            FilmSimulation::Acros { filter } => match filter {
                MonochromaticFilter::Std => "acrosstd_1.png",
                MonochromaticFilter::Red => "acrosr_1.png",
                MonochromaticFilter::Green => "acrosg_1.png",
                MonochromaticFilter::Yellow => "acrosye_1.png",
            },
        };

        Ok(to_value(img).unwrap())
    })
}
