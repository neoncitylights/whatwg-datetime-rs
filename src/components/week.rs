use crate::tokens::{TOKEN_ABBR_WEEK, TOKEN_HYPHEN};
use crate::utils::{collect_ascii_digits, week_number_of_year};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWeek {
	pub(crate) year: i32,
	pub(crate) week: u8,
}

impl YearWeek {
	#[inline]
	pub(crate) fn new(year: i32, week: u8) -> Self {
		Self { year, week }
	}
}

pub fn parse_week(input: &str) -> Option<YearWeek> {
	// Step 1, 2
	let mut position = 0usize;

	// Step 3, 4
	let year_string = collect_ascii_digits(input, &mut position);
	let year = year_string.parse::<i32>().unwrap();
	if year <= 0 {
		return None;
	}

	// Step 5
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_HYPHEN) {
		return None;
	} else {
		position += 1;
	}

	// Step 6
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_ABBR_WEEK) {
		return None;
	} else {
		position += 1;
	}

	// Step 7
	let parsed_week = collect_ascii_digits(input, &mut position);
	if parsed_week.len() != 2 {
		return None;
	}

	let week = parsed_week.parse::<u8>().unwrap();
	let max_weeks = week_number_of_year(year)?;
	if week < 1 || week > max_weeks {
		return None;
	}

	Some(YearWeek::new(year, week))
}

#[cfg(test)]
mod tests {
	use super::{parse_week, YearWeek};

	#[test]
	fn test_parse_week() {
		assert_eq!(parse_week("2004-W53"), Some(YearWeek::new(2004, 53)));
	}

	#[test]
	fn test_parse_week_fails_year_is_zero() {
		assert_eq!(parse_week("0000-W01"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_separator() {
		assert_eq!(parse_week("2004_W01"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_abbr() {
		assert_eq!(parse_week("2003-𝓌01"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_length() {
		assert_eq!(parse_week("2004-W1"), None);
		assert_eq!(parse_week("2008-W001"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_num_lower_bound() {
		assert_eq!(parse_week("2022-W00"), None);
		assert_eq!(parse_week("1897-W00"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_num_upper_bound() {
		assert_eq!(parse_week("2004-W54"), None);
		assert_eq!(parse_week("1996-W53"), None);
	}
}
