# ssn

U.S. Social Security Number (SSN) parsing and validation for Rust.

[![Crates.io](https://img.shields.io/crates/v/ssn.svg)](https://crates.io/crates/ssn)
[![Documentation](https://docs.rs/ssn/badge.svg)](https://docs.rs/ssn)
[![CI](https://github.com/gideonsolutions/ssn/actions/workflows/ci.yml/badge.svg)](https://github.com/gideonsolutions/ssn/actions/workflows/ci.yml)

## Usage

```rust
use ssn::Ssn;

// Parse from string with dashes
let ssn: Ssn = "123-45-6789".parse().unwrap();
assert_eq!(ssn.to_string(), "123-45-6789");

// Parse from string without dashes
let ssn: Ssn = "123456789".parse().unwrap();
assert_eq!(ssn.to_string(), "123-45-6789");

// Access components
assert_eq!(ssn.area(), 123);
assert_eq!(ssn.group(), 45);
assert_eq!(ssn.serial(), 6789);

// Create from components
let ssn = Ssn::new(123, 45, 6789).unwrap();
```

## Validation

Validates per [SSA rules](https://secure.ssa.gov/poms.nsf/lnx/0110201035):

- Area number (first 3 digits) cannot be 000, 666, or 900-999
- Group number (middle 2 digits) cannot be 00
- Serial number (last 4 digits) cannot be 0000

```rust
use ssn::{Ssn, SsnParseError};

// Invalid area
assert!(matches!(
    "000-12-3456".parse::<Ssn>(),
    Err(SsnParseError::InvalidArea(0))
));

// Invalid format
assert!(matches!(
    "123+45-6789".parse::<Ssn>(),
    Err(SsnParseError::InvalidFormat)
));
```

## Privacy

The `Debug` implementation masks sensitive digits:

```rust
let ssn: Ssn = "123-45-6789".parse().unwrap();
assert_eq!(format!("{:?}", ssn), "Ssn(XXX-XX-6789)");
```

## License

Apache-2.0
