//! U.S. Social Security Number (SSN) validation.

use core::fmt;
use core::str::FromStr;

use crate::{ParseError, parse_components};

/// A validated U.S. Social Security Number.
///
/// # Validation
///
/// Per [SSA rules](https://secure.ssa.gov/poms.nsf/lnx/0110201035):
/// - Area number (first 3 digits) must be 001–665 or 667–899
/// - Group number (middle 2 digits) must be 01–99
/// - Serial number (last 4 digits) must be 0001–9999
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ssn {
    area: u16,
    group: u8,
    serial: u16,
}

impl Ssn {
    /// Creates a new SSN from its components.
    pub fn new(area: u16, group: u8, serial: u16) -> Result<Self, ParseError> {
        Self::validate(area, group, serial)?;
        Ok(Self {
            area,
            group,
            serial,
        })
    }

    fn validate(area: u16, group: u8, serial: u16) -> Result<(), ParseError> {
        if area == 0 || area == 666 || area > 899 {
            return Err(ParseError::InvalidArea(area));
        }
        if group == 0 || group > 99 {
            return Err(ParseError::InvalidGroup(group));
        }
        if serial == 0 || serial > 9999 {
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

impl FromStr for Ssn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area, group, serial) = parse_components(s)?;
        Self::new(area, group, serial)
    }
}

impl fmt::Display for Ssn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03}-{:02}-{:04}", self.area, self.group, self.serial)
    }
}

impl fmt::Debug for Ssn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ssn(XXX-XX-{:04})", self.serial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ssn_with_dashes() {
        let ssn: Ssn = "123-45-6789".parse().unwrap();
        assert_eq!(ssn.area(), 123);
        assert_eq!(ssn.group(), 45);
        assert_eq!(ssn.serial(), 6789);
        assert_eq!(ssn.to_string(), "123-45-6789");
    }

    #[test]
    fn valid_ssn_no_dashes() {
        let ssn: Ssn = "123456789".parse().unwrap();
        assert_eq!(ssn.to_string(), "123-45-6789");
    }

    #[test]
    fn valid_boundary_low() {
        let ssn = Ssn::new(1, 1, 1).unwrap();
        assert_eq!(ssn.to_string(), "001-01-0001");
    }

    #[test]
    fn valid_boundary_665() {
        let ssn = Ssn::new(665, 99, 9999).unwrap();
        assert_eq!(ssn.to_string(), "665-99-9999");
    }

    #[test]
    fn valid_boundary_667() {
        let ssn = Ssn::new(667, 1, 1).unwrap();
        assert_eq!(ssn.to_string(), "667-01-0001");
    }

    #[test]
    fn valid_boundary_899() {
        let ssn = Ssn::new(899, 99, 9999).unwrap();
        assert_eq!(ssn.to_string(), "899-99-9999");
    }

    #[test]
    fn invalid_area_000() {
        assert!(matches!(
            "000-45-6789".parse::<Ssn>(),
            Err(ParseError::InvalidArea(0))
        ));
    }

    #[test]
    fn invalid_area_666() {
        assert!(matches!(
            "666-45-6789".parse::<Ssn>(),
            Err(ParseError::InvalidArea(666))
        ));
    }

    #[test]
    fn invalid_area_900() {
        assert!(matches!(
            "900-45-6789".parse::<Ssn>(),
            Err(ParseError::InvalidArea(900))
        ));
    }

    #[test]
    fn invalid_group_00() {
        assert!(matches!(
            "123-00-6789".parse::<Ssn>(),
            Err(ParseError::InvalidGroup(0))
        ));
    }

    #[test]
    fn invalid_serial_0000() {
        assert!(matches!(
            "123-45-0000".parse::<Ssn>(),
            Err(ParseError::InvalidSerial(0))
        ));
    }

    #[test]
    fn invalid_area_out_of_bounds() {
        assert!(matches!(
            Ssn::new(1000, 45, 6789),
            Err(ParseError::InvalidArea(1000))
        ));
    }

    #[test]
    fn invalid_group_out_of_bounds() {
        assert!(matches!(
            Ssn::new(123, 100, 6789),
            Err(ParseError::InvalidGroup(100))
        ));
    }

    #[test]
    fn invalid_serial_out_of_bounds() {
        assert!(matches!(
            Ssn::new(123, 45, 10000),
            Err(ParseError::InvalidSerial(10000))
        ));
    }

    #[test]
    fn debug_masks_sensitive_data() {
        let ssn: Ssn = "123-45-6789".parse().unwrap();
        assert_eq!(format!("{ssn:?}"), "Ssn(XXX-XX-6789)");
    }

    #[test]
    fn invalid_format() {
        assert!(matches!(
            "12a-45-6789".parse::<Ssn>(),
            Err(ParseError::InvalidFormat(_))
        ));
    }
}
