//! U.S. Individual Taxpayer Identification Number (ITIN) validation.

use core::fmt;
use core::str::FromStr;

use crate::{ParseError, parse_components};

/// A validated U.S. Individual Taxpayer Identification Number.
///
/// # Validation
///
/// Per [IRS rules](https://www.irs.gov/individuals/individual-taxpayer-identification-number):
/// - Area number (first 3 digits) must be 900–999
/// - Group number (middle 2 digits) must be 50–65, 70–88, 90–92, or 94–99
/// - Serial number (last 4 digits) may be 0000–9999
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Itin {
    area: u16,
    group: u8,
    serial: u16,
}

/// Returns `true` if the group number is in the valid ITIN range.
pub(crate) fn is_valid_itin_group(group: u8) -> bool {
    matches!(group, 50..=65 | 70..=88 | 90..=92 | 94..=99)
}

impl Itin {
    /// Creates a new ITIN from its components.
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
        if !is_valid_itin_group(group) {
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

impl FromStr for Itin {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area, group, serial) = parse_components(s)?;
        Self::new(area, group, serial)
    }
}

impl fmt::Display for Itin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03}-{:02}-{:04}", self.area, self.group, self.serial)
    }
}

impl fmt::Debug for Itin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Itin(XXX-XX-{:04})", self.serial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_itin_with_dashes() {
        let itin: Itin = "900-70-1234".parse().unwrap();
        assert_eq!(itin.area(), 900);
        assert_eq!(itin.group(), 70);
        assert_eq!(itin.serial(), 1234);
        assert_eq!(itin.to_string(), "900-70-1234");
    }

    #[test]
    fn valid_itin_no_dashes() {
        let itin: Itin = "999881234".parse().unwrap();
        assert_eq!(itin.to_string(), "999-88-1234");
    }

    #[test]
    fn valid_itin_group_boundaries() {
        // Group 50-65
        assert!(Itin::new(900, 50, 0).is_ok());
        assert!(Itin::new(900, 65, 0).is_ok());
        // Group 70-88
        assert!(Itin::new(900, 70, 0).is_ok());
        assert!(Itin::new(900, 88, 0).is_ok());
        // Group 90-92
        assert!(Itin::new(900, 90, 0).is_ok());
        assert!(Itin::new(900, 92, 0).is_ok());
        // Group 94-99
        assert!(Itin::new(900, 94, 0).is_ok());
        assert!(Itin::new(900, 99, 0).is_ok());
    }

    #[test]
    fn valid_itin_serial_zero() {
        // ITIN allows serial 0000
        let itin = Itin::new(900, 70, 0).unwrap();
        assert_eq!(itin.to_string(), "900-70-0000");
    }

    #[test]
    fn invalid_area_below_900() {
        assert!(matches!(
            Itin::new(899, 70, 1234),
            Err(ParseError::InvalidArea(899))
        ));
    }

    #[test]
    fn invalid_area_above_999() {
        assert!(matches!(
            Itin::new(1000, 70, 1234),
            Err(ParseError::InvalidArea(1000))
        ));
    }

    #[test]
    fn invalid_group_not_in_itin_range() {
        // Group 93 is ATIN, not ITIN
        assert!(matches!(
            Itin::new(900, 93, 1234),
            Err(ParseError::InvalidGroup(93))
        ));
        // Group 49 is below ITIN range
        assert!(matches!(
            Itin::new(900, 49, 1234),
            Err(ParseError::InvalidGroup(49))
        ));
        // Group 66-69 are not valid
        assert!(matches!(
            Itin::new(900, 66, 1234),
            Err(ParseError::InvalidGroup(66))
        ));
        assert!(matches!(
            Itin::new(900, 69, 1234),
            Err(ParseError::InvalidGroup(69))
        ));
        // Group 89 is not valid
        assert!(matches!(
            Itin::new(900, 89, 1234),
            Err(ParseError::InvalidGroup(89))
        ));
    }

    #[test]
    fn invalid_serial_out_of_bounds() {
        assert!(matches!(
            Itin::new(900, 70, 10000),
            Err(ParseError::InvalidSerial(10000))
        ));
    }

    #[test]
    fn debug_masks_sensitive_data() {
        let itin: Itin = "900-70-1234".parse().unwrap();
        assert_eq!(format!("{itin:?}"), "Itin(XXX-XX-1234)");
    }
}
