# tin

U.S. Taxpayer Identification Number (TIN) parsing and validation for Rust.

Supports **SSN** (Social Security Number), **ITIN** (Individual Taxpayer Identification Number), and **ATIN** (Adoption Taxpayer Identification Number) — all of which share the `XXX-XX-XXXX` format.

[![Crates.io](https://img.shields.io/crates/v/tin.svg)](https://crates.io/crates/tin)
[![Documentation](https://docs.rs/tin/badge.svg)](https://docs.rs/tin)
[![CI](https://github.com/gideonsolutions/tin/actions/workflows/ci.yml/badge.svg)](https://github.com/gideonsolutions/tin/actions/workflows/ci.yml)

## Usage

### Parse a specific type

```rust
use tin::Ssn;

let ssn: Ssn = "123-45-6789".parse().unwrap();
assert_eq!(ssn.to_string(), "123-45-6789");
assert_eq!(ssn.area(), 123);
assert_eq!(ssn.group(), 45);
assert_eq!(ssn.serial(), 6789);
```

```rust
use tin::Itin;

let itin: Itin = "900-70-1234".parse().unwrap();
assert_eq!(itin.to_string(), "900-70-1234");
```

```rust
use tin::Atin;

let atin: Atin = "900-93-5678".parse().unwrap();
assert_eq!(atin.to_string(), "900-93-5678");
```

### Auto-detect type

```rust
use tin::Tin;

let tin: Tin = "123-45-6789".parse().unwrap();
assert!(matches!(tin, Tin::Ssn(_)));

let tin: Tin = "900-70-1234".parse().unwrap();
assert!(matches!(tin, Tin::Itin(_)));

let tin: Tin = "900-93-5678".parse().unwrap();
assert!(matches!(tin, Tin::Atin(_)));
```

### Create from components

```rust
use tin::{Ssn, Itin, Atin};

let ssn = Ssn::new(123, 45, 6789).unwrap();
let itin = Itin::new(900, 70, 1234).unwrap();
let atin = Atin::new(900, 93, 5678).unwrap();
```

## Validation Rules

| Type | Area | Group | Serial |
|------|------|-------|--------|
| SSN | 001–665, 667–899 | 01–99 | 0001–9999 |
| ITIN | 900–999 | 50–65, 70–88, 90–92, 94–99 | 0000–9999 |
| ATIN | 900–999 | 93 | 0000–9999 |

```rust
use tin::{Ssn, ParseError};

// Invalid area
assert!(matches!(
    "000-12-3456".parse::<Ssn>(),
    Err(ParseError::InvalidArea(0))
));

// Invalid format
assert!(matches!(
    "123+45-6789".parse::<Ssn>(),
    Err(ParseError::InvalidFormat(_))
));
```

## Privacy

The `Debug` implementation masks sensitive digits:

```rust
use tin::{Ssn, Itin, Atin};

let ssn: Ssn = "123-45-6789".parse().unwrap();
assert_eq!(format!("{:?}", ssn), "Ssn(XXX-XX-6789)");

let itin: Itin = "900-70-1234".parse().unwrap();
assert_eq!(format!("{:?}", itin), "Itin(XXX-XX-1234)");

let atin: Atin = "900-93-5678".parse().unwrap();
assert_eq!(format!("{:?}", atin), "Atin(XXX-XX-5678)");
```

## License

Apache-2.0
