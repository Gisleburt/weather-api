use juniper::GraphQLEnum;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, GraphQLEnum, Serialize)]
pub enum Visibility {
    Unknown,
    VeryPoor,
    Poor,
    Moderate,
    Good,
    VeryGood,
    Excellent,
}

#[derive(Error, Debug, PartialEq)]
pub enum VisibilityCodeConversionError {
    #[error("invalid visibility code, found {0}")]
    InvalidCode(String),
}

impl FromStr for Visibility {
    type Err = VisibilityCodeConversionError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "UN" => Ok(Visibility::Unknown),
            "VP" => Ok(Visibility::VeryPoor),
            "PO" => Ok(Visibility::Poor),
            "MO" => Ok(Visibility::Moderate),
            "GO" => Ok(Visibility::Good),
            "VG" => Ok(Visibility::VeryGood),
            "EX" => Ok(Visibility::Excellent),
            _ => Err(VisibilityCodeConversionError::InvalidCode(code.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_visibility_codes() {
        assert_eq!(Visibility::from_str("UN").unwrap(), Visibility::Unknown);
        assert_eq!(Visibility::from_str("VP").unwrap(), Visibility::VeryPoor);
        assert_eq!(Visibility::from_str("PO").unwrap(), Visibility::Poor);
        assert_eq!(Visibility::from_str("MO").unwrap(), Visibility::Moderate);
        assert_eq!(Visibility::from_str("GO").unwrap(), Visibility::Good);
        assert_eq!(Visibility::from_str("VG").unwrap(), Visibility::VeryGood);
        assert_eq!(Visibility::from_str("EX").unwrap(), Visibility::Excellent);
    }

    #[test]
    fn test_unknown_code() {
        assert_eq!(
            Visibility::from_str("An invalid code").unwrap_err(),
            VisibilityCodeConversionError::InvalidCode("An invalid code".to_string())
        );
    }
}
