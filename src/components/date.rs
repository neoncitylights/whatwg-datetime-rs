use crate::utils::{collect_ascii_digits, max_days_in_month_year};
use crate::{parse_month_component, TOKEN_DATETIME_SEPARATOR};

pub fn parse_date_string(s: &str) -> Option<(u32, u8, u8)> {
	let mut position = 0usize;
	let parsed = parse_date_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_date_component(s: &str, position: &mut usize) -> Option<(u32, u8, u8)> {
	let (year, month) = parse_month_component(s, position)?;

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

	Some((year, month, day))
}

#[cfg(test)]
mod tests {
	use super::parse_date_component;

	#[test]
	fn test_parse_date_component() {
		let mut position = 0usize;
		let parsed = parse_date_component("2004-12-31", &mut position);

		assert_eq!(parsed, Some((2004, 12, 31)));
	}
}
