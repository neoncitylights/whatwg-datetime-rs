# `whatwg-datetime`

[![License](https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue)](#license)
[![CI](https://github.com/nlp-rs/rust-template/actions/workflows/main.yml/badge.svg)](https://github.com/nlp-rs/rust-template/actions/workflows/main.yml)
[![Security audit](https://github.com/nlp-rs/rust-template/actions/workflows/security-audit.yml/badge.svg)](https://github.com/nlp-rs/rust-template/actions/workflows/security-audit.yml)
[![codecov](https://codecov.io/github/acmuta-research/whatwg-datetime-rs/branch/main/graph/badge.svg?token=p3VPg9QCaE)](https://codecov.io/github/acmuta-research/whatwg-datetime-rs)

A Rust crate for parsing the datetime microsyntax, as defined by the WHATWG HTML Standard.

## Install

```shell
cargo add whatwg-datetime
```

## Usage

This library currently implements 8 of the 9 datetime formats defined by the WHATWG HTML Standard. The only format not implemented is the duration format, which is tracked in [issue #29](https://github.com/acmuta-research/whatwg-datetime-rs/issues/29).

```rust
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use whatwg_datetime::parse_global_datetime;

assert_eq!(
	parse_global_datetime("2011-11-18T14:54Z"),
	Some(DateTime::<Utc>::from_utc(
		NaiveDateTime::new(
			NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
			NaiveTime::from_hms_opt(14, 54, 0).unwrap(),
		),
		Utc,
	))
);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
