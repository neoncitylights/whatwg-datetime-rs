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
	let month = parsed_month.parse::<u8>().ok()?;
	if !is_valid_month(&month) {
		return None;
	}

	Some((year, month))
}

#[cfg(test)]
mod tests {
	use super::parse_month_component;

	#[test]
	fn test_parse_month_component() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-12", &mut position);

		assert_eq!(parsed, Some((2004, 12)));
	}
}
