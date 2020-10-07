use thiserror::Error;
use std::convert::TryFrom;

type UvCode = str;

#[derive(Debug, Eq, PartialEq)]
struct UvIndex(u8);

impl UvIndex {
    fn describe(&self) -> &str {
        match self.0 {
            0..=2 => "Low exposure. No protection required. You can safely stay outside",
            3..=5 => "Moderate exposure. Seek shade during midday hours, cover up and wear sunscreen",
            6..=7 => "High exposure. Seek shade during midday hours, cover up and wear sunscreen",
            8..=10 => "Very high. Avoid being outside during midday hours. Shirt, sunscreen and hat are essential",
            11..=u8::MAX => "Extreme. Avoid being outside during midday hours. Shirt, sunscreen and hat essential.",
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UvCodeConversionError {
    #[error("invalid uv index, expected positive integer, found {0}")]
    InvalidCode(String),
}

impl TryFrom<&UvCode> for UvIndex {
    type Error = UvCodeConversionError;

    fn try_from(code: &UvCode) -> Result<Self, Self::Error> {
        let index: u8 = code.parse().map_err(|_| UvCodeConversionError::InvalidCode(code.to_string()))?;
        Ok(UvIndex(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uv_code_to_index() {
        assert_eq!(UvIndex::try_from("1").unwrap(), UvIndex(1));
        assert_eq!(UvIndex::try_from("2").unwrap(), UvIndex(2));
        assert_eq!(UvIndex::try_from("3").unwrap(), UvIndex(3));
        assert_eq!(UvIndex::try_from("4").unwrap(), UvIndex(4));
        assert_eq!(UvIndex::try_from("5").unwrap(), UvIndex(5));
        assert_eq!(UvIndex::try_from("6").unwrap(), UvIndex(6));
        assert_eq!(UvIndex::try_from("7").unwrap(), UvIndex(7));
        assert_eq!(UvIndex::try_from("8").unwrap(), UvIndex(8));
        assert_eq!(UvIndex::try_from("9").unwrap(), UvIndex(9));
        assert_eq!(UvIndex::try_from("10").unwrap(), UvIndex(10));
        assert_eq!(UvIndex::try_from("11").unwrap(), UvIndex(11));
        assert_eq!(UvIndex::try_from("255").unwrap(), UvIndex(255));
    }

    #[test]
    fn test_invalid_uv_code() {
        assert_eq!(
            UvIndex::try_from("Not a number").err().unwrap(),
            UvCodeConversionError::InvalidCode("Not a number".to_string())
        );
    }
}
