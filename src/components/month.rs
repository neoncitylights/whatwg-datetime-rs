use crate::utils::{collect_ascii_digits, is_valid_month};
use crate::TOKEN_DATETIME_SEPARATOR;

pub fn parse_month_string(s: &str) -> Option<(u32, u8)> {
	let mut position = 0usize;
	let parsed = parse_month_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_month_component(s: &str, position: &mut usize) -> Option<(u32, u8)> {
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

	Some((year, month))
}

#[cfg(test)]
mod tests {
	use super::{parse_month_component, parse_month_string};

	#[test]
	fn test_parse_month_string() {
		let parsed = parse_month_string("2004-12");
		assert_eq!(parsed, Some((2004, 12)));
	}

	#[test]
	fn test_parse_month_string_fails_invalid_month() {
		let parsed = parse_month_string("2004-2a");
		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_string_fails() {
		let parsed = parse_month_string("2004-13");
		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_month_component() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-12", &mut position);

		assert_eq!(parsed, Some((2004, 12)));
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
