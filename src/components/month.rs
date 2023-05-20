use crate::utils::{collect_ascii_digits, is_valid_month};
use crate::TOKEN_DATETIME_SEPARATOR;

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

pub fn parse_month(s: &str) -> Option<YearMonth> {
	let mut position = 0usize;
	let parsed = parse_month_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_month_component(s: &str, position: &mut usize) -> Option<YearMonth> {
	let parsed_year = collect_ascii_digits(s, position);
	if parsed_year.len() < 4 {
		return None;
	}

	let year = parsed_year.parse::<u32>().ok()?;
	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_month = collect_ascii_digits(s, position);
	if parsed_month.len() != 2 {
		return None;
	}

	let month = parsed_month.parse::<u8>().ok()?;
	if !is_valid_month(&month) {
		return None;
	}

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
