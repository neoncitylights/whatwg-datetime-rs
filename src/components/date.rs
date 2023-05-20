use crate::utils::{collect_ascii_digits, max_days_in_month_year};
use crate::{parse_month_component, TOKEN_DATETIME_SEPARATOR};
use chrono::NaiveDate;

pub fn parse_date(s: &str) -> Option<NaiveDate> {
	let mut position = 0usize;
	let parsed = parse_date_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_date_component(s: &str, position: &mut usize) -> Option<NaiveDate> {
	let year_month = parse_month_component(s, position)?;
	let year = year_month.year;
	let month = year_month.month;

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_day = collect_ascii_digits(s, position);
	if parsed_day.len() != 2 {
		return None;
	}

	let max_day = max_days_in_month_year(month, year)?;
	let day = parsed_day.parse::<u8>().ok()?;

	if !(1..=max_day).contains(&day) {
		return None;
	}

	NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
	use super::parse_date;
	use chrono::NaiveDate;

	#[test]
	fn test_parse_date() {
		assert_eq!(
			parse_date("2011-11-18"),
			NaiveDate::from_ymd_opt(2011, 11, 18)
		);
	}

	#[test]
	fn test_parse_date_leap_year() {
		assert_eq!(
			parse_date("2012-02-29"),
			NaiveDate::from_ymd_opt(2012, 2, 29)
		);
	}

	#[test]
	fn test_parse_date_fails_not_leap_year() {
		assert_eq!(parse_date("2007-02-29"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_month() {
		assert_eq!(parse_date("2011-00-19"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_day_length() {
		assert_eq!(parse_date("2011-11-0"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_day_upper_bound() {
		assert_eq!(parse_date("2011-11-32"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_separator() {
		assert_eq!(parse_date("2011-11/19"), None);
	}
}
