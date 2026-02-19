//! U.S. Adoption Taxpayer Identification Number (ATIN) validation.

use core::fmt;
use core::str::FromStr;

use crate::{ParseError, parse_components};

/// A validated U.S. Adoption Taxpayer Identification Number.
///
/// # Validation
///
/// Per [IRS rules](https://www.irs.gov/individuals/adoption-taxpayer-identification-number):
/// - Area number (first 3 digits) must be 900–999
/// - Group number (middle 2 digits) must be 93
/// - Serial number (last 4 digits) may be 0000–9999
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Atin {
    area: u16,
    group: u8,
    serial: u16,
}

impl Atin {
    /// Creates a new ATIN from its components.
    pub fn new(area: u16, group: u8, serial: u16) -> Result<Self, ParseError> {
        Self::validate(area, group, serial)?;
        Ok(Self {
            area,
            group,
            serial,
        })
    }

    fn validate(area: u16, group: u8, serial: u16) -> Result<(), ParseError> {
        if !(900..=999).contains(&area) {
            return Err(ParseError::InvalidArea(area));
        }
        if group != 93 {
            return Err(ParseError::InvalidGroup(group));
        }
        if serial > 9999 {
            return Err(ParseError::InvalidSerial(serial));
        }
        Ok(())
    }

    /// Returns the area number (first 3 digits).
    pub fn area(&self) -> u16 {
        self.area
    }

    /// Returns the group number (middle 2 digits).
    pub fn group(&self) -> u8 {
        self.group
    }

    /// Returns the serial number (last 4 digits).
    pub fn serial(&self) -> u16 {
        self.serial
    }
}

impl FromStr for Atin {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area, group, serial) = parse_components(s)?;
        Self::new(area, group, serial)
    }
}

impl fmt::Display for Atin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03}-{:02}-{:04}", self.area, self.group, self.serial)
    }
}

impl fmt::Debug for Atin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Atin(XXX-XX-{:04})", self.serial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_atin_with_dashes() {
        let atin: Atin = "900-93-1234".parse().unwrap();
        assert_eq!(atin.area(), 900);
        assert_eq!(atin.group(), 93);
        assert_eq!(atin.serial(), 1234);
        assert_eq!(atin.to_string(), "900-93-1234");
    }

    #[test]
    fn valid_atin_no_dashes() {
        let atin: Atin = "999931234".parse().unwrap();
        assert_eq!(atin.to_string(), "999-93-1234");
    }

    #[test]
    fn valid_atin_serial_zero() {
        let atin = Atin::new(900, 93, 0).unwrap();
        assert_eq!(atin.to_string(), "900-93-0000");
    }

    #[test]
    fn valid_atin_area_boundaries() {
        assert!(Atin::new(900, 93, 0).is_ok());
        assert!(Atin::new(999, 93, 0).is_ok());
    }

    #[test]
    fn invalid_area_below_900() {
        assert!(matches!(
            Atin::new(899, 93, 1234),
            Err(ParseError::InvalidArea(899))
        ));
    }

    #[test]
    fn invalid_area_above_999() {
        assert!(matches!(
            Atin::new(1000, 93, 1234),
            Err(ParseError::InvalidArea(1000))
        ));
    }

    #[test]
    fn invalid_group_not_93() {
        assert!(matches!(
            Atin::new(900, 70, 1234),
            Err(ParseError::InvalidGroup(70))
        ));
        assert!(matches!(
            Atin::new(900, 92, 1234),
            Err(ParseError::InvalidGroup(92))
        ));
        assert!(matches!(
            Atin::new(900, 94, 1234),
            Err(ParseError::InvalidGroup(94))
        ));
    }

    #[test]
    fn invalid_serial_out_of_bounds() {
        assert!(matches!(
            Atin::new(900, 93, 10000),
            Err(ParseError::InvalidSerial(10000))
        ));
    }

    #[test]
    fn debug_masks_sensitive_data() {
        let atin: Atin = "900-93-1234".parse().unwrap();
        assert_eq!(format!("{atin:?}"), "Atin(XXX-XX-1234)");
    }
}
