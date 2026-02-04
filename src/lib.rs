//! U.S. Social Security Number (SSN) parsing and validation.
//!
//! # Example
//!
//! ```
//! use ssn::Ssn;
//!
//! let ssn: Ssn = "123-45-6789".parse().unwrap();
//! assert_eq!(ssn.to_string(), "123-45-6789");
//!
//! // Also accepts format without dashes
//! let ssn: Ssn = "123456789".parse().unwrap();
//! assert_eq!(ssn.to_string(), "123-45-6789");
//! ```

use core::fmt;
use core::str::FromStr;
use regex::Regex;

/// Matches SSN in format XXX-XX-XXXX or XXXXXXXXX (9 consecutive digits)
static SSN_PATTERN: &str = r"^(\d{3})-(\d{2})-(\d{4})$|^(\d{9})$";

/// A validated U.S. Social Security Number.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ssn {
    area: u16,
    group: u8,
    serial: u16,
}

/// Errors that can occur when parsing an SSN.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SsnParseError {
    /// The input string does not match the expected SSN format.
    ///
    /// SSNs must be in either `XXX-XX-XXXX` (with dashes) or `XXXXXXXXX` (9 consecutive digits) format.
    #[error("invalid format '{0}': expected XXX-XX-XXXX or XXXXXXXXX")]
    InvalidFormat(String),
    /// The area number (first 3 digits) is invalid per Social Security Administration rules.
    ///
    /// Valid area numbers are 001-665 and 667-899. Area 000, 666, and 900-999 are not assigned.
    #[error("invalid area number: {0} (must be 001-665 or 667-899)")]
    InvalidArea(u16),
    /// The group number (middle 2 digits) is invalid per Social Security Administration rules.
    ///
    /// Valid group numbers are 01-99. Group 00 is not assigned.
    #[error("invalid group number: {0} (must be 01-99)")]
    InvalidGroup(u8),
    /// The serial number (last 4 digits) is invalid per Social Security Administration rules.
    ///
    /// Valid serial numbers are 0001-9999. Serial 0000 is not assigned.
    #[error("invalid serial number: {0} (must be 0001-9999)")]
    InvalidSerial(u16),
}

impl Ssn {
    /// Creates a new SSN from its components.
    ///
    /// Returns an error if the components violate Social Security Administration rules.
    pub fn new(area: u16, group: u8, serial: u16) -> Result<Self, SsnParseError> {
        Self::validate(area, group, serial)?;
        Ok(Self {
            area,
            group,
            serial,
        })
    }

    /// Validates SSN components per Social Security Administration rules.
    ///
    /// Per <https://secure.ssa.gov/poms.nsf/lnx/0110201035>:
    /// - Area number (first 3 digits) must be 001-665, 667-899 (not 000, 666, or 900-999)
    /// - Group number (middle 2 digits) must be 01-99 (not 00)
    /// - Serial number (last 4 digits) must be 0001-9999 (not 0000)
    fn validate(area: u16, group: u8, serial: u16) -> Result<(), SsnParseError> {
        if area == 0 || area == 666 || area > 899 {
            return Err(SsnParseError::InvalidArea(area));
        }
        if group == 0 || group > 99 {
            return Err(SsnParseError::InvalidGroup(group));
        }
        if serial == 0 || serial > 9999 {
            return Err(SsnParseError::InvalidSerial(serial));
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
    type Err = SsnParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(SSN_PATTERN)
            .expect("SSN_PATTERN is a valid regex: two alternates for dashed and undashed formats");
        let caps = re
            .captures(s)
            .ok_or_else(|| SsnParseError::InvalidFormat(s.to_owned()))?;

        let (area, group, serial) =
            if let (Some(a), Some(g), Some(s)) = (caps.get(1), caps.get(2), caps.get(3)) {
                (a.as_str(), g.as_str(), s.as_str())
            } else if let Some(full) = caps.get(4) {
                let full = full.as_str();
                (&full[0..3], &full[3..5], &full[5..9])
            } else {
                return Err(SsnParseError::InvalidFormat(s.to_owned()));
            };

        let area: u16 = area
            .parse()
            .expect("area is exactly three ASCII digits as enforced by SSN_PATTERN; parse::<u16> cannot fail");
        let group: u8 = group.parse().expect(
            "group is exactly two ASCII digits as enforced by SSN_PATTERN; parse::<u8> cannot fail",
        );
        let serial: u16 = serial
            .parse()
            .expect("serial is exactly four ASCII digits as enforced by SSN_PATTERN; parse::<u16> cannot fail");

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
    fn invalid_format_with_plus() {
        let input = "213+21-2342";
        let result: Result<Ssn, _> = input.parse();
        assert_eq!(result, Err(SsnParseError::InvalidFormat(input.to_owned())));
    }

    #[test]
    fn invalid_format_with_asterisks() {
        let input = "213*13*3322*";
        let result: Result<Ssn, _> = input.parse();
        assert_eq!(result, Err(SsnParseError::InvalidFormat(input.to_owned())));
    }

    #[test]
    fn invalid_format_mixed_separators() {
        let input = "123-456789";
        let result: Result<Ssn, _> = input.parse();
        assert_eq!(result, Err(SsnParseError::InvalidFormat(input.to_owned())));
    }

    #[test]
    fn invalid_format_partial_dashes() {
        let input = "123-45-678";
        let result: Result<Ssn, _> = input.parse();
        assert_eq!(result, Err(SsnParseError::InvalidFormat(input.to_owned())));
    }

    #[test]
    fn invalid_format_letters() {
        let input = "12a-45-6789";
        let result: Result<Ssn, _> = input.parse();
        assert_eq!(result, Err(SsnParseError::InvalidFormat(input.to_owned())));
    }

    #[test]
    fn invalid_area_000() {
        let result: Result<Ssn, _> = "000-45-6789".parse();
        assert!(matches!(result, Err(SsnParseError::InvalidArea(0))));
    }

    #[test]
    fn invalid_area_666() {
        let result: Result<Ssn, _> = "666-45-6789".parse();
        assert!(matches!(result, Err(SsnParseError::InvalidArea(666))));
    }

    #[test]
    fn invalid_area_900_999() {
        let result: Result<Ssn, _> = "900-45-6789".parse();
        assert!(matches!(result, Err(SsnParseError::InvalidArea(900))));

        let result: Result<Ssn, _> = "999-45-6789".parse();
        assert!(matches!(result, Err(SsnParseError::InvalidArea(999))));
    }

    #[test]
    fn invalid_group_00() {
        let result: Result<Ssn, _> = "123-00-6789".parse();
        assert!(matches!(result, Err(SsnParseError::InvalidGroup(0))));
    }

    #[test]
    fn invalid_serial_0000() {
        let result: Result<Ssn, _> = "123-45-0000".parse();
        assert!(matches!(result, Err(SsnParseError::InvalidSerial(0))));
    }

    #[test]
    fn invalid_area_out_of_bounds() {
        let result = Ssn::new(1000, 45, 6789);
        assert!(matches!(result, Err(SsnParseError::InvalidArea(1000))));
    }

    #[test]
    fn invalid_group_out_of_bounds() {
        let result = Ssn::new(123, 100, 6789);
        assert!(matches!(result, Err(SsnParseError::InvalidGroup(100))));
    }

    #[test]
    fn invalid_serial_out_of_bounds() {
        let result = Ssn::new(123, 45, 10000);
        assert!(matches!(result, Err(SsnParseError::InvalidSerial(10000))));
    }

    #[test]
    fn debug_masks_sensitive_data() {
        let ssn: Ssn = "123-45-6789".parse().unwrap();
        assert_eq!(format!("{:?}", ssn), "Ssn(XXX-XX-6789)");
    }
}
