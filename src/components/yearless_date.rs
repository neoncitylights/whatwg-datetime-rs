use crate::tokens::TOKEN_HYPHEN;
use crate::{collect_day_and_validate, collect_month_and_validate, parse_value};
use whatwg_infra::collect_codepoints;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearlessDate {
	pub(crate) month: u8,
	pub(crate) day: u8,
}

impl YearlessDate {
	#[inline]
	pub(crate) fn new(month: u8, day: u8) -> Self {
		Self { month, day }
	}
}

#[inline]
pub fn parse_yearless_date(s: &str) -> Option<YearlessDate> {
	parse_value(s, parse_yearless_date_component)
}

pub fn parse_yearless_date_component(s: &str, position: &mut usize) -> Option<YearlessDate> {
	let collected = collect_codepoints(s, position, |c| c == TOKEN_HYPHEN);
	if !matches!(collected.len(), 0 | 2) {
		return None;
	}

	let month = collect_month_and_validate(s, position)?;
	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_HYPHEN) {
		return None;
	} else {
		*position += 1;
	}

	let day = collect_day_and_validate(s, position, month)?;
	Some(YearlessDate::new(month, day))
}

#[cfg(test)]
mod tests {
	#[rustfmt::skip]
	use super::{
		parse_yearless_date,
		parse_yearless_date_component,
		YearlessDate,
	};

	#[test]
	fn test_parse_yearless_date() {
		assert_eq!(
			parse_yearless_date("11-18"),
			Some(YearlessDate::new(11, 18))
		);
	}

	#[test]
	fn test_parse_yearless_date_fails_empty_string() {
		assert_eq!(parse_yearless_date(""), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_separator() {
		assert_eq!(parse_yearless_date("11/18"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_month_upper_bound() {
		assert_eq!(parse_yearless_date("13-01"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_month_length() {
		assert_eq!(parse_yearless_date("1-01"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_day_lower_bound() {
		assert_eq!(parse_yearless_date("01-00"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_day_upper_bound() {
		assert_eq!(parse_yearless_date("01-32"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_day_length() {
		assert_eq!(parse_yearless_date("01-9"), None);
	}

	#[test]
	fn test_parse_yearless_date_component() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("12-31", &mut position);

		assert_eq!(parsed, Some(YearlessDate::new(12, 31)));
	}

	#[test]
	fn test_parse_yearless_date_component_fails_empty_string() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_yearless_date_only_one_separator() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("-", &mut position);

		assert_eq!(parsed, None);
	}
}
