pub mod exif_meta;
// pub mod fujifilm;
pub mod fujifilm;
pub mod photo;
pub mod tag;
// use crate::models::fujifilm::{
//     DRangePriority, DynamicRange, GrainSize, GrainStrength, SettingStrength, TransSensor,
// };
// use rocket::serde::{Deserialize, Serialize};
// use strum_macros::{Display as EnumDisplay, EnumString};

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(transparent)]
pub struct Timestamp(i64);

//
// #[derive(Clone, Debug, EnumString, EnumDisplay)]
// pub enum WB {
//     Auto,
//     AutoWhitePriority,
//     AmbiencePriority,
//     Custom1,
//     Custom2,
//     Custom3,
//     ColorTemperature,
//     Daylight,
//     Shade,
//     FluorescentLight1,
//     FluorescentLight2,
//     FluorescentLight3,
//     Incandescent,
//     Underwater,
// }
//
// #[derive(Clone, Debug, EnumString, EnumDisplay)]
// pub enum FilmSim {
//     ProviaStandard,
//     VelviaVivid,
//     AstiaSoft,
//     ClassicChrome,
//     ClassicNeg,
//     NostalgicNeg,
//     ProNegHi,
//     ProNegStd,
//     EternaCinema,
//     RealaAce,
//     Sepia,
//     Acros,
//     AcrosYellow,
//     AcrosRed,
//     AcrosGreen,
//     Monochrome,
//     MonochromeYellow,
//     MonochromeRed,
//     MonochromeGreen,
// }
