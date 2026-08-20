use crate::model::Error;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DoseType {
    Physical,
    Effective,
    Error,
    Coded,
}

impl FromStr for DoseType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PHYSICAL" => Ok(DoseType::Physical),
            "EFFECTIVE" => Ok(DoseType::Effective),
            "ERROR" => Ok(DoseType::Error),
            "CODED" => Ok(DoseType::Coded),
            &_ => Err(Error::DoseTypeFromStr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dose_type_from_str() {
        let r = DoseType::from_str("PHYSICAL");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), DoseType::Physical);

        let r = DoseType::from_str("EFFECTIVE");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), DoseType::Effective);

        let r = DoseType::from_str("ERROR");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), DoseType::Error);

        let r = DoseType::from_str("CODED");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), DoseType::Coded);
    }

    fn test_dose_type_from_str_err() {
        let r = DoseType::from_str("Undefined");
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(e, Error::DoseTypeFromStr);
    }
}
