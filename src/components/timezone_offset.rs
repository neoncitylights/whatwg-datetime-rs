use crate::parse_value;
use crate::tokens::{TOKEN_COLON, TOKEN_MINUS, TOKEN_PLUS, TOKEN_Z};
use crate::utils::collect_ascii_digits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeZoneOffset {
	pub(crate) hours: i8,
	pub(crate) minutes: i8,
}

impl TimeZoneOffset {
	#[inline]
	pub(crate) fn new(hours: i8, minutes: i8) -> Self {
		Self { hours, minutes }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneSign {
	Positive,
	Negative,
}

impl TryFrom<char> for TimeZoneSign {
	type Error = ();
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			TOKEN_PLUS => Ok(TimeZoneSign::Positive),
			TOKEN_MINUS => Ok(TimeZoneSign::Negative),
			_ => Err(()),
		}
	}
}

#[inline]
pub fn parse_timezone_offset(s: &str) -> Option<TimeZoneOffset> {
	parse_value(s, parse_timezone_offset_component)
}

pub fn parse_timezone_offset_component(s: &str, position: &mut usize) -> Option<TimeZoneOffset> {
	let char_at = s.chars().nth(*position);

	let mut minutes = 0i8;
	let mut hours = 0i8;

	match char_at {
		Some(TOKEN_Z) => {
			*position += 1;
		}
		Some(TOKEN_PLUS) | Some(TOKEN_MINUS) => {
			let sign = TimeZoneSign::try_from(char_at.unwrap()).ok().unwrap();
			*position += 1;

			let collected = collect_ascii_digits(s, position);
			let collected_len = collected.len();
			if collected_len == 2 {
				hours = collected.parse::<i8>().unwrap();
				if *position > s.len()
					|| s.chars().nth(*position) != Some(TOKEN_COLON)
				{
					return None;
				} else {
					*position += 1;
				}

				let parsed_mins = collect_ascii_digits(s, position);
				if parsed_mins.len() != 2 {
					return None;
				}

				minutes = parsed_mins.parse::<i8>().unwrap();
			} else if collected_len == 4 {
				let (hour_str, min_str) = collected.split_at(2);
				hours = hour_str.parse::<i8>().unwrap();
				minutes = min_str.parse::<i8>().unwrap();
			} else {
				return None;
			}

			if !(0..=23).contains(&hours) {
				return None;
			}

			if !(0..=59).contains(&minutes) {
				return None;
			}

			if sign == TimeZoneSign::Negative {
				hours *= -1;
				minutes *= -1;
			}
		}
		_ => (),
	}

	Some(TimeZoneOffset::new(hours, minutes))
}

#[cfg(test)]
mod tests {
	#[rustfmt::skip]
	use super::{
		parse_timezone_offset,
		parse_timezone_offset_component,
		TimeZoneOffset,
		TimeZoneSign,
	};

	#[test]
	pub fn test_parse_timezone_sign_tryfrom_char_positive() {
		let parsed = TimeZoneSign::try_from('+');
		assert_eq!(parsed, Ok(TimeZoneSign::Positive));
	}

	#[test]
	pub fn test_parse_timezone_sign_tryfrom_char_negative() {
		let parsed = TimeZoneSign::try_from('-');
		assert_eq!(parsed, Ok(TimeZoneSign::Negative));
	}

	#[test]
	pub fn test_parse_timezone_sign_tryfrom_char_fails() {
		let parsed = TimeZoneSign::try_from('a');
		assert_eq!(parsed, Err(()));
	}

	#[test]
	pub fn test_parse_timezone_offset() {
		let parsed = parse_timezone_offset("+01:00");
		assert_eq!(parsed, Some(TimeZoneOffset::new(1, 0)));
	}

	#[test]
	pub fn test_parse_timezone_offset_z() {
		let parsed = parse_timezone_offset("Z");
		assert_eq!(parsed, Some(TimeZoneOffset::new(0, 0)));
	}

	#[test]
	pub fn test_parse_timezone_offset_plus_1_hour_colon() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+01:00", &mut position);

		assert_eq!(parsed, Some(TimeZoneOffset::new(1, 0)));
	}

	#[test]
	pub fn test_parse_timezone_offset_neg_1_hour_colon() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01:00", &mut position);

		assert_eq!(parsed, Some(TimeZoneOffset::new(-1, 0)));
	}

	#[test]
	pub fn test_parse_timezone_offset_plus_1_hour_no_delim() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+0100", &mut position);

		assert_eq!(parsed, Some(TimeZoneOffset::new(1, 0)));
	}

	#[test]
	fn parse_timezone_offset_component_neg_1_hour_no_delim() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-0100", &mut position);

		assert_eq!(parsed, Some(TimeZoneOffset::new(-1, 0)));
	}

	#[test]
	fn parse_timezone_offset_fails_invalid_min_length() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-010", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn parse_timezone_offset_fails_colon_invalid_length_empty() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01:", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn parse_timezone_offset_fails_colon_invalid_length() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01:0", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn parse_timezone_offset_fails_invalid_length() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01000", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn parse_timezone_offset_fails_invalid_hour_upper_bound() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+24:00", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn parse_timezone_offset_fails_invalid_minute_upper_bound() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-00:67", &mut position);

		assert_eq!(parsed, None);
	}
}
