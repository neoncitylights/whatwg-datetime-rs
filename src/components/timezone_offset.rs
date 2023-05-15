use crate::utils::collect_ascii_digits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeZoneOffset {
	pub(crate) hours: i8,
	pub(crate) minutes: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneSign {
	Positive,
	Negative,
}

pub fn parse_timezone_offset_component(s: &str, position: &mut usize) -> Option<TimeZoneOffset> {
	let char_at = s.chars().nth(*position);

	let mut minutes = 0i8;
	let mut hours = 0i8;

	match char_at {
		Some('Z') => {
			*position += 1;
		}
		Some('+') | Some('-') => {
			let sign = match char_at {
				Some('+') => TimeZoneSign::Positive,
				Some('-') => TimeZoneSign::Negative,
				_ => unreachable!(),
			};

			*position += 1;

			let collected = collect_ascii_digits(s, position);
			let collected_len = collected.len();
			if collected_len == 2 {
				hours = collected.parse::<i8>().unwrap();
				if *position > s.len() || s.chars().nth(*position) != Some(':') {
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

	Some(TimeZoneOffset { hours, minutes })
}

#[cfg(test)]
mod tests {
	use super::{parse_timezone_offset_component, TimeZoneOffset};

	#[test]
	pub fn test_parse_timezone_offset_component() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("Z", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 0,
				minutes: 0
			})
		);
	}

	#[test]
	pub fn test_parse_timezone_offset_plus_1_hour_colon() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+01:00", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 1,
				minutes: 0
			})
		);
	}

	#[test]
	pub fn test_parse_timezone_offset_neg_1_hour_colon() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01:00", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: -1,
				minutes: 0
			})
		);
	}

	#[test]
	pub fn test_parse_timezone_offset_plus_1_hour_no_delim() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 1,
				minutes: 0
			})
		);
	}

	#[test]
	fn parse_timezone_offset_component_neg_1_hour_no_delim() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: -1,
				minutes: 0
			})
		);
	}

	#[test]
	fn parse_timezone_offset_fails_invalid_min_length() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-010", &mut position);

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
