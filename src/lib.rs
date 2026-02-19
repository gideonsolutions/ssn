//! U.S. Taxpayer Identification Number (TIN) parsing and validation.
//!
//! This crate supports three TIN types that share the `XXX-XX-XXXX` format:
//!
//! - **SSN** — Social Security Number (area 001–665, 667–899)
//! - **ITIN** — Individual Taxpayer Identification Number (area 900–999, specific groups)
//! - **ATIN** — Adoption Taxpayer Identification Number (area 900–999, group 93)
//!
//! # Example
//!
//! ```
//! use tin::{Tin, Ssn, Itin, Atin};
//!
//! // Parse a specific type
//! let ssn: Ssn = "123-45-6789".parse().unwrap();
//! assert_eq!(ssn.to_string(), "123-45-6789");
//!
//! // Auto-detect type via the Tin enum
//! let tin: Tin = "900-70-1234".parse().unwrap();
//! assert!(matches!(tin, Tin::Itin(_)));
//! ```

mod atin;
mod itin;
mod ssn;

use core::fmt;
use core::str::FromStr;

use regex::Regex;

pub use atin::Atin;
pub use itin::Itin;
pub use ssn::Ssn;

/// Matches the `XXX-XX-XXXX` or `XXXXXXXXX` format shared by SSN, ITIN, and ATIN.
static TIN_PATTERN: &str = r"^(\d{3})-(\d{2})-(\d{4})$|^(\d{9})$";

/// Errors that can occur when parsing a TIN.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    /// The input string does not match the expected format.
    #[error("invalid format: expected XXX-XX-XXXX or XXXXXXXXX")]
    InvalidFormat(String),
    /// The area number (first 3 digits) is invalid for the target type.
    #[error("invalid area number: {0}")]
    InvalidArea(u16),
    /// The group number (middle 2 digits) is invalid for the target type.
    #[error("invalid group number: {0}")]
    InvalidGroup(u8),
    /// The serial number (last 4 digits) is invalid for the target type.
    #[error("invalid serial number: {0}")]
    InvalidSerial(u16),
}

/// Parses a `XXX-XX-XXXX` or `XXXXXXXXX` string into `(area, group, serial)` components.
pub fn parse_components(s: &str) -> Result<(u16, u8, u16), ParseError> {
    let re = Regex::new(TIN_PATTERN)
        .expect("TIN_PATTERN is a valid regex: two alternates for dashed and undashed formats");
    let caps = re
        .captures(s)
        .ok_or_else(|| ParseError::InvalidFormat(s.to_owned()))?;

    let (area, group, serial) =
        if let (Some(a), Some(g), Some(s)) = (caps.get(1), caps.get(2), caps.get(3)) {
            (a.as_str(), g.as_str(), s.as_str())
        } else if let Some(full) = caps.get(4) {
            let full = full.as_str();
            (&full[0..3], &full[3..5], &full[5..9])
        } else {
            return Err(ParseError::InvalidFormat(s.to_owned()));
        };

    let area: u16 = area.parse().expect(
        "area is exactly three ASCII digits as enforced by TIN_PATTERN; parse::<u16> cannot fail",
    );
    let group: u8 = group.parse().expect(
        "group is exactly two ASCII digits as enforced by TIN_PATTERN; parse::<u8> cannot fail",
    );
    let serial: u16 = serial.parse().expect(
        "serial is exactly four ASCII digits as enforced by TIN_PATTERN; parse::<u16> cannot fail",
    );

    Ok((area, group, serial))
}

/// A U.S. Taxpayer Identification Number that auto-detects its type.
///
/// The `Tin` enum wraps [`Ssn`], [`Itin`], and [`Atin`], selecting the correct
/// variant based on the area and group numbers.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Tin {
    /// Social Security Number.
    Ssn(Ssn),
    /// Individual Taxpayer Identification Number.
    Itin(Itin),
    /// Adoption Taxpayer Identification Number.
    Atin(Atin),
}

impl Tin {
    /// Returns the area number (first 3 digits).
    pub fn area(&self) -> u16 {
        match self {
            Tin::Ssn(v) => v.area(),
            Tin::Itin(v) => v.area(),
            Tin::Atin(v) => v.area(),
        }
    }

    /// Returns the group number (middle 2 digits).
    pub fn group(&self) -> u8 {
        match self {
            Tin::Ssn(v) => v.group(),
            Tin::Itin(v) => v.group(),
            Tin::Atin(v) => v.group(),
        }
    }

    /// Returns the serial number (last 4 digits).
    pub fn serial(&self) -> u16 {
        match self {
            Tin::Ssn(v) => v.serial(),
            Tin::Itin(v) => v.serial(),
            Tin::Atin(v) => v.serial(),
        }
    }
}

impl FromStr for Tin {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area, group, serial) = parse_components(s)?;

        match area {
            // SSN range: 001-665, 667-899
            1..=665 | 667..=899 => Ok(Tin::Ssn(Ssn::new(area, group, serial)?)),
            // TIN range 900-999: ATIN if group == 93, else try ITIN
            900..=999 if group == 93 => Ok(Tin::Atin(Atin::new(area, group, serial)?)),
            900..=999 if itin::is_valid_itin_group(group) => {
                Ok(Tin::Itin(Itin::new(area, group, serial)?))
            }
            // Invalid: area 0, 666, or 900-999 with invalid group
            _ => {
                if area == 0 || area == 666 {
                    Err(ParseError::InvalidArea(area))
                } else {
                    Err(ParseError::InvalidGroup(group))
                }
            }
        }
    }
}

impl fmt::Display for Tin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tin::Ssn(v) => v.fmt(f),
            Tin::Itin(v) => v.fmt(f),
            Tin::Atin(v) => v.fmt(f),
        }
    }
}

impl fmt::Debug for Tin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tin::Ssn(v) => v.fmt(f),
            Tin::Itin(v) => v.fmt(f),
            Tin::Atin(v) => v.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Format parsing ---

    #[test]
    fn parse_components_dashed() {
        let (a, g, s) = parse_components("123-45-6789").unwrap();
        assert_eq!((a, g, s), (123, 45, 6789));
    }

    #[test]
    fn parse_components_undashed() {
        let (a, g, s) = parse_components("123456789").unwrap();
        assert_eq!((a, g, s), (123, 45, 6789));
    }

    #[test]
    fn parse_components_invalid_format() {
        assert!(matches!(
            parse_components("12a-45-6789"),
            Err(ParseError::InvalidFormat(_))
        ));
        assert!(matches!(
            parse_components("123-456789"),
            Err(ParseError::InvalidFormat(_))
        ));
        assert!(matches!(
            parse_components(""),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    // --- Tin auto-detection ---

    #[test]
    fn tin_detects_ssn() {
        let tin: Tin = "123-45-6789".parse().unwrap();
        assert!(matches!(tin, Tin::Ssn(_)));
        assert_eq!(tin.area(), 123);
        assert_eq!(tin.group(), 45);
        assert_eq!(tin.serial(), 6789);
    }

    #[test]
    fn tin_detects_itin() {
        let tin: Tin = "900-70-1234".parse().unwrap();
        assert!(matches!(tin, Tin::Itin(_)));
    }

    #[test]
    fn tin_detects_atin() {
        let tin: Tin = "900-93-1234".parse().unwrap();
        assert!(matches!(tin, Tin::Atin(_)));
    }

    #[test]
    fn tin_invalid_area_000() {
        assert!(matches!(
            "000-45-6789".parse::<Tin>(),
            Err(ParseError::InvalidArea(0))
        ));
    }

    #[test]
    fn tin_invalid_area_666() {
        assert!(matches!(
            "666-45-6789".parse::<Tin>(),
            Err(ParseError::InvalidArea(666))
        ));
    }

    #[test]
    fn tin_invalid_group_in_900_range() {
        // Group 10 is not valid for any 900-range type
        assert!(matches!(
            "900-10-1234".parse::<Tin>(),
            Err(ParseError::InvalidGroup(10))
        ));
    }

    #[test]
    fn tin_display_delegates() {
        let tin: Tin = "123-45-6789".parse().unwrap();
        assert_eq!(tin.to_string(), "123-45-6789");
    }

    #[test]
    fn tin_debug_delegates() {
        let tin: Tin = "123-45-6789".parse().unwrap();
        assert_eq!(format!("{tin:?}"), "Ssn(XXX-XX-6789)");

        let tin: Tin = "900-70-1234".parse().unwrap();
        assert_eq!(format!("{tin:?}"), "Itin(XXX-XX-1234)");

        let tin: Tin = "900-93-5678".parse().unwrap();
        assert_eq!(format!("{tin:?}"), "Atin(XXX-XX-5678)");
    }

    #[test]
    fn tin_ssn_boundary_667() {
        let tin: Tin = "667-01-0001".parse().unwrap();
        assert!(matches!(tin, Tin::Ssn(_)));
    }

    #[test]
    fn tin_ssn_boundary_899() {
        let tin: Tin = "899-99-9999".parse().unwrap();
        assert!(matches!(tin, Tin::Ssn(_)));
    }
}
