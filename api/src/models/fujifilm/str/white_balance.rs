use crate::models::fujifilm::{
    str::{Error, ParseKey},
    WBShift, WhiteBalance,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;

impl Display for WhiteBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WhiteBalance::Auto { shift } => write!(f, "{}", format!("Auto {}", shift.to_string())),
            WhiteBalance::AutoWhitePriority { shift } => {
                write!(f, "{}", format!("AutoWhitePriority {}", shift.to_string()))
            }
            WhiteBalance::AutoAmbiencePriority { shift } => write!(
                f,
                "{}",
                format!("AutoAmbiencePriority {}", shift.to_string())
            ),
            WhiteBalance::Custom1 { shift } => {
                write!(f, "{}", format!("Custom1 {}", shift.to_string()))
            }
            WhiteBalance::Custom2 { shift } => {
                write!(f, "{}", format!("Custom2 {}", shift.to_string()))
            }
            WhiteBalance::Custom3 { shift } => {
                write!(f, "{}", format!("Custom3 {}", shift.to_string()))
            }
            WhiteBalance::Daylight { shift } => {
                write!(f, "{}", format!("Daylight {}", shift.to_string()))
            }
            WhiteBalance::Cloudy { shift } => {
                write!(f, "{}", format!("Cloudy {}", shift.to_string()))
            }
            WhiteBalance::FluorescentLight1 { shift } => {
                write!(f, "{}", format!("FluorescentLight1 {}", shift.to_string()))
            }
            WhiteBalance::FluorescentLight2 { shift } => {
                write!(f, "{}", format!("FluorescentLight2 {}", shift.to_string()))
            }
            WhiteBalance::FluorescentLight3 { shift } => {
                write!(f, "{}", format!("FluorescentLight3 {}", shift.to_string()))
            }
            WhiteBalance::Incandescent { shift } => {
                write!(f, "{}", format!("Incandescent {}", shift.to_string()))
            }
            WhiteBalance::Underwater { shift } => {
                write!(f, "{}", format!("Underwater {}", shift.to_string()))
            }
            WhiteBalance::Kelvin { temperature, shift } => {
                write!(f, "{}", format!("{}K {}", temperature, shift.to_string()))
            }
        }
    }
}

impl FromStr for WhiteBalance {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-9]+)K$").unwrap());
        let caps = RE.captures(s);

        if let Some(cap) = caps {
            let temp = cap.get(1).unwrap();
            let temp: i32 = temp.as_str().to_string().parse::<i32>().unwrap();

            return Ok(WhiteBalance::Kelvin {
                temperature: temp,
                shift: WBShift::default(),
            });
        };

        match s {
            "Auto" => Ok(WhiteBalance::Auto {
                shift: WBShift::default(),
            }),
            "AutoWhitePriority" => Ok(WhiteBalance::AutoWhitePriority {
                shift: WBShift::default(),
            }),
            "AutoAmbiencePriority" => Ok(WhiteBalance::AutoAmbiencePriority {
                shift: WBShift::default(),
            }),
            "Custom1" => Ok(WhiteBalance::Custom1 {
                shift: WBShift::default(),
            }),
            "Custom2" => Ok(WhiteBalance::Custom2 {
                shift: WBShift::default(),
            }),
            "Custom3" => Ok(WhiteBalance::Custom3 {
                shift: WBShift::default(),
            }),
            "Daylight" => Ok(WhiteBalance::Daylight {
                shift: WBShift::default(),
            }),
            "Cloudy" => Ok(WhiteBalance::Cloudy {
                shift: WBShift::default(),
            }),
            "FluorescentLight1" => Ok(WhiteBalance::FluorescentLight1 {
                shift: WBShift::default(),
            }),
            "FluorescentLight2" => Ok(WhiteBalance::FluorescentLight2 {
                shift: WBShift::default(),
            }),
            "FluorescentLight3" => Ok(WhiteBalance::FluorescentLight3 {
                shift: WBShift::default(),
            }),
            "Incandescent" => Ok(WhiteBalance::Incandescent {
                shift: WBShift::default(),
            }),
            "Underwater" => Ok(WhiteBalance::Underwater {
                shift: WBShift::default(),
            }),

            _ => Err(Error::Parse {
                key: ParseKey::WhiteBalance,
                reason: format!("Could not match WB to {}", s),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::fujifilm::str::{Error, ParseKey};

    #[test]
    fn it_converts_wb_to_string() {
        let wb = WhiteBalance::Auto {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Auto R2, B-3");

        let wb = WhiteBalance::AutoWhitePriority {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "AutoWhitePriority R2, B-3");

        let wb = WhiteBalance::AutoAmbiencePriority {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "AutoAmbiencePriority R2, B-3");

        let wb = WhiteBalance::Custom1 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Custom1 R2, B-3");

        let wb = WhiteBalance::Custom2 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Custom2 R2, B-3");

        let wb = WhiteBalance::Custom3 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Custom3 R2, B-3");

        let wb = WhiteBalance::Daylight {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Daylight R2, B-3");

        let wb = WhiteBalance::Cloudy {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Cloudy R2, B-3");

        let wb = WhiteBalance::FluorescentLight1 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "FluorescentLight1 R2, B-3");

        let wb = WhiteBalance::FluorescentLight2 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "FluorescentLight2 R2, B-3");

        let wb = WhiteBalance::FluorescentLight3 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "FluorescentLight3 R2, B-3");

        let wb = WhiteBalance::Incandescent {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Incandescent R2, B-3");

        let wb = WhiteBalance::Underwater {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Underwater R2, B-3");

        let wb = WhiteBalance::Kelvin {
            temperature: 9500,
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "9500K R2, B-3");
    }

    #[test]
    fn it_parses_from_str() {
        assert_eq!(
            WhiteBalance::from_str("Auto"),
            Ok(WhiteBalance::Auto {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("AutoWhitePriority"),
            Ok(WhiteBalance::AutoWhitePriority {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("AutoAmbiencePriority"),
            Ok(WhiteBalance::AutoAmbiencePriority {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Custom1"),
            Ok(WhiteBalance::Custom1 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Custom2"),
            Ok(WhiteBalance::Custom2 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Custom3"),
            Ok(WhiteBalance::Custom3 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Daylight"),
            Ok(WhiteBalance::Daylight {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Cloudy"),
            Ok(WhiteBalance::Cloudy {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("FluorescentLight1"),
            Ok(WhiteBalance::FluorescentLight1 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("FluorescentLight2"),
            Ok(WhiteBalance::FluorescentLight2 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("FluorescentLight3"),
            Ok(WhiteBalance::FluorescentLight3 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Incandescent"),
            Ok(WhiteBalance::Incandescent {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Underwater"),
            Ok(WhiteBalance::Underwater {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("9360K"),
            Ok(WhiteBalance::Kelvin {
                temperature: 9360,
                shift: WBShift::default()
            })
        );
    }
}
