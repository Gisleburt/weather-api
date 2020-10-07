use thiserror::Error;
use std::convert::TryFrom;

type VisibilityCode = str;

#[derive(Debug, Eq, PartialEq)]
enum Visibility {
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

impl TryFrom<&VisibilityCode> for Visibility {
    type Error = VisibilityCodeConversionError;

    fn try_from(code: &str) -> Result<Self, Self::Error> {
        match code {
            "UN" => Ok(Visibility::Unknown),
            "VP" => Ok(Visibility::VeryPoor),
            "PO" => Ok(Visibility::Poor),
            "MO" => Ok(Visibility::Moderate),
            "GO" => Ok(Visibility::Good),
            "VG" => Ok(Visibility::VeryGood),
            "EX" => Ok(Visibility::Excellent),
            _ => Err(VisibilityCodeConversionError::InvalidCode(code.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_visibility_codes() {
        assert_eq!(Visibility::try_from("UN").unwrap(), Visibility::Unknown);
        assert_eq!(Visibility::try_from("VP").unwrap(), Visibility::VeryPoor);
        assert_eq!(Visibility::try_from("PO").unwrap(), Visibility::Poor);
        assert_eq!(Visibility::try_from("MO").unwrap(), Visibility::Moderate);
        assert_eq!(Visibility::try_from("GO").unwrap(), Visibility::Good);
        assert_eq!(Visibility::try_from("VG").unwrap(), Visibility::VeryGood);
        assert_eq!(Visibility::try_from("EX").unwrap(), Visibility::Excellent);
    }


    #[test]
    fn test_unknown_code() {
        let result = Visibility::try_from("An invalid code");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), VisibilityCodeConversionError::InvalidCode("An invalid code".to_string()));
    }
}

