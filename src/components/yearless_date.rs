use whatwg_infra::collect_codepoints;

use crate::utils::{collect_ascii_digits, is_valid_month, max_days_in_month_year};
use crate::TOKEN_DATETIME_SEPARATOR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearlessDate {
	pub(crate) month: u8,
	pub(crate) day: u8,
}

pub fn parse_yearless_date(s: &str) -> Option<YearlessDate> {
	let mut position = 0usize;
	let parsed = parse_yearless_date_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_yearless_date_component(s: &str, position: &mut usize) -> Option<YearlessDate> {
	let collected = collect_codepoints(s, position, |c| c == TOKEN_DATETIME_SEPARATOR);
	if !matches!(collected.len(), 0 | 2) {
		return None;
	}

	let parsed_month = collect_ascii_digits(s, position);
	if parsed_month.len() != 2 {
		return None;
	}

	let month = parsed_month.parse::<u8>().ok()?;
	if !is_valid_month(&month) {
		return None;
	}

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_day = collect_ascii_digits(s, position);
	if parsed_day.len() != 2 {
		return None;
	}

	let day = parsed_day.parse::<u8>().ok()?;
	let max_days = max_days_in_month_year(month, 4).unwrap();
	if !(1..=max_days).contains(&day) {
		return None;
	}

	Some(YearlessDate { month, day })
}

#[cfg(test)]
mod tests {
	#[rustfmt::skip]
	use super::{
		parse_yearless_date_component,
		YearlessDate,
	};

	#[test]
	fn test_parse_yearless_date_component() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("12-31", &mut position);

		assert_eq!(parsed, Some(YearlessDate { month: 12, day: 31 }));
	}
}
