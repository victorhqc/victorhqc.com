use crate::models::fujifilm::ToneCurve;
use std::fmt::{Display, Formatter};

impl Display for ToneCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("H{} S{}", self.highlights, self.shadows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_to_string() {
        let expected = ToneCurve {
            highlights: 1.5,
            shadows: 2.0,
        }
        .to_string();

        assert_eq!(expected, "H1.5 S2");
    }

    #[test]
    fn it_handles_negative_numbers() {
        let expected = ToneCurve {
            highlights: -2.0,
            shadows: -3.5,
        }.to_string();

        assert_eq!(expected, "H-2 S-3.5");
    }
}
