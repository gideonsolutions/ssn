# Changelog

## [1.0.0](https://github.com/gideonsolutions/tin/compare/v0.1.0...v1.0.0) (2026-02-19)


### ⚠ BREAKING CHANGES

* crate renamed from ssn to tin; SsnParseError renamed to ParseError

### Features

* rename crate to tin with SSN, ITIN, and ATIN support ([#12](https://github.com/gideonsolutions/tin/issues/12)) ([62b2ce6](https://github.com/gideonsolutions/tin/commit/62b2ce64413f858b12b8ffb20f6f38673dcfcef3))

## 0.1.0 (2026-02-19)

### Features

* Initial TIN parsing and validation library with SSN, ITIN, and ATIN support
* Auto-detection of TIN type via the `Tin` enum
* Privacy-masked `Debug` implementation for all types
