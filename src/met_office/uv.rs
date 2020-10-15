use juniper::GraphQLObject;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

type UvNumberType = i32;

#[derive(Debug, Eq, PartialEq, Serialize, GraphQLObject)]
pub struct UvIndex {
    index: UvNumberType,
    description: &'static str,
}

impl UvIndex {
    fn index_to_description(index: UvNumberType) -> &'static str {
        match index {
            UvNumberType::MIN..=2 => "Low exposure. No protection required. You can safely stay outside",
            3..=5 => "Moderate exposure. Seek shade during midday hours, cover up and wear sunscreen",
            6..=7 => "High exposure. Seek shade during midday hours, cover up and wear sunscreen",
            8..=10 => "Very high. Avoid being outside during midday hours. Shirt, sunscreen and hat are essential",
            11..=UvNumberType::MAX => "Extreme. Avoid being outside during midday hours. Shirt, sunscreen and hat essential.",
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UvCodeConversionError {
    #[error("invalid uv index, expected positive integer, found {0}")]
    InvalidCode(String),
}

impl FromStr for UvIndex {
    type Err = UvCodeConversionError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let index = code
            .parse()
            .map_err(|_| UvCodeConversionError::InvalidCode(code.to_string()))?;
        let description = UvIndex::index_to_description(index);
        Ok(UvIndex { index, description })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uv_code_to_index() {
        assert_eq!(UvIndex::from_str("1").unwrap().index, 1);
        assert_eq!(
            UvIndex::from_str("1").unwrap().description,
            "Low exposure. No protection required. You can safely stay outside"
        );

        assert_eq!(UvIndex::from_str("2").unwrap().index, 2);
        assert_eq!(
            UvIndex::from_str("2").unwrap().description,
            "Low exposure. No protection required. You can safely stay outside"
        );

        assert_eq!(UvIndex::from_str("3").unwrap().index, 3);
        assert_eq!(
            UvIndex::from_str("3").unwrap().description,
            "Moderate exposure. Seek shade during midday hours, cover up and wear sunscreen"
        );

        assert_eq!(UvIndex::from_str("4").unwrap().index, 4);
        assert_eq!(
            UvIndex::from_str("4").unwrap().description,
            "Moderate exposure. Seek shade during midday hours, cover up and wear sunscreen"
        );

        assert_eq!(UvIndex::from_str("5").unwrap().index, 5);
        assert_eq!(
            UvIndex::from_str("5").unwrap().description,
            "Moderate exposure. Seek shade during midday hours, cover up and wear sunscreen"
        );

        assert_eq!(UvIndex::from_str("6").unwrap().index, 6);
        assert_eq!(
            UvIndex::from_str("6").unwrap().description,
            "High exposure. Seek shade during midday hours, cover up and wear sunscreen"
        );

        assert_eq!(UvIndex::from_str("7").unwrap().index, 7);
        assert_eq!(
            UvIndex::from_str("7").unwrap().description,
            "High exposure. Seek shade during midday hours, cover up and wear sunscreen"
        );

        assert_eq!(UvIndex::from_str("8").unwrap().index, 8);
        assert_eq!(
            UvIndex::from_str("8").unwrap().description,
            "Very high. Avoid being outside during midday hours. Shirt, sunscreen and hat are essential"
        );

        assert_eq!(UvIndex::from_str("9").unwrap().index, 9);
        assert_eq!(
            UvIndex::from_str("9").unwrap().description,
            "Very high. Avoid being outside during midday hours. Shirt, sunscreen and hat are essential"
        );

        assert_eq!(UvIndex::from_str("10").unwrap().index, 10);
        assert_eq!(
            UvIndex::from_str("10").unwrap().description,
            "Very high. Avoid being outside during midday hours. Shirt, sunscreen and hat are essential"
        );

        assert_eq!(UvIndex::from_str("11").unwrap().index, 11);
        assert_eq!(
            UvIndex::from_str("11").unwrap().description,
            "Extreme. Avoid being outside during midday hours. Shirt, sunscreen and hat essential."
        );

        assert_eq!(UvIndex::from_str("255").unwrap().index, 255);
        assert_eq!(
            UvIndex::from_str("255").unwrap().description,
            "Extreme. Avoid being outside during midday hours. Shirt, sunscreen and hat essential."
        );
    }

    #[test]
    fn test_invalid_uv_code() {
        assert_eq!(
            UvIndex::from_str("Not a number").err().unwrap(),
            UvCodeConversionError::InvalidCode("Not a number".to_string())
        );
    }
}
