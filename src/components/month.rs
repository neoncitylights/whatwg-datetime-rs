use crate::tokens::TOKEN_HYPHEN;
use crate::utils::collect_ascii_digits;
use crate::{collect_month_and_validate, parse_format};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearMonth {
	pub(crate) year: u32,
	pub(crate) month: u8,
}

impl YearMonth {
	pub(crate) fn new(year: u32, month: u8) -> Self {
		Self { year, month }
	}
}

/// Parse a [proleptic-Gregorian date][proleptic-greg] consisting of a year and a month,
/// with no time-zone or date information
///
/// This follows the rules for [parsing a month string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.1 Months][whatwg-html-months].
///
/// # Examples
/// // ```
/// //use whatwg_datetime::{parse_month, YearMonth};
///
/// // assert_eq!(
/// //    parse_month("2011-11"),
/// //    Some(YearMonth::new(2011, 11))
/// //);
/// // ```
///
/// [proleptic-greg]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#proleptic-gregorian-date
/// [whatwg-html-months]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#months
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-month-string
#[inline]
pub fn parse_month(s: &str) -> Option<YearMonth> {
	parse_format(s, parse_month_component)
}

pub fn parse_month_component(s: &str, position: &mut usize) -> Option<YearMonth> {
	let parsed_year = collect_ascii_digits(s, position);
	if parsed_year.len() < 4 {
		return None;
	}

	let year = parsed_year.parse::<u32>().ok()?;
	if year == 0 {
		return None;
	}

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_HYPHEN) {
		return None;
	} else {
		*position += 1;
	}

	let month = collect_month_and_validate(s, position)?;
	Some(YearMonth::new(year, month))
}

#[cfg(test)]
mod tests {
	use super::{parse_month, parse_month_component, YearMonth};

	#[test]
	fn test_parse_month_string() {
		let parsed = parse_month("2004-12");
		assert_eq!(parsed, Some(YearMonth::new(2004, 12)));
	}

	#[test]
	fn test_parse_month_string_fails_invalid_month() {
		let parsed = parse_month("2004-2a");
		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_string_fails() {
		let parsed = parse_month("2004-13");
		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_component() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-12", &mut position);

		assert_eq!(parsed, Some(YearMonth::new(2004, 12)));
	}

	#[test]
	fn test_parse_month_component_fails_year_lt_4_digits() {
		let mut position = 0usize;
		let parsed = parse_month_component("200-12", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_component_fails_invalid_month_lower_bound() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-0", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_component_fails_invalid_month_upper_bound() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-13", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_component_fails_invalid_month_syntax() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-1a", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_component_fails_invalid_separator() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004/12", &mut position);

		assert_eq!(parsed, None);
	}
}
