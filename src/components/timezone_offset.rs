use crate::parse_format;
use crate::tokens::{TOKEN_COLON, TOKEN_MINUS, TOKEN_PLUS, TOKEN_Z};
use crate::utils::collect_ascii_digits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeZoneOffset {
	pub(crate) hours: i32,
	pub(crate) minutes: i32,
}

impl TimeZoneOffset {
	#[inline]
	pub(crate) fn new(hours: i32, minutes: i32) -> Self {
		Self { hours, minutes }
	}

	/// Creates a new `TimeZoneOffset` from a signed number of hours and minutes.
	///
	/// This asserts that:
	///  - hours are in between -23 and 23, inclusive,
	///  - minutes are in between 0 and 59, inclusive
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::TimeZoneOffset;
	///
	/// assert!(TimeZoneOffset::new_opt(-7, 0).is_some());
	/// assert!(TimeZoneOffset::new_opt(23, 59).is_some());
	/// assert!(TimeZoneOffset::new_opt(24, 0).is_none()); // Hours must be between [-23, 23]
	/// assert!(TimeZoneOffset::new_opt(1, 60).is_none()); // Minutes must be between [0, 59]
	/// ```
	pub fn new_opt(hours: i32, minutes: i32) -> Option<Self> {
		if !(-23..=23).contains(&hours) {
			return None;
		}

		if !(0..=59).contains(&minutes) {
			return None;
		}

		Some(Self::new(hours, minutes))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeZoneSign {
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

/// Parse a time-zone offset, with a signed number of hours and minutes
///
/// This follows the rules for [parsing a time-zone offset string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.6 Time zoness][whatwg-html-tzoffset].
///
/// # Examples
/// ```
/// use whatwg_datetime::{parse_timezone_offset, TimeZoneOffset};
///
/// // Parse a local datetime string with a date,
/// // a T delimiter, anda  time with fractional seconds
/// assert_eq!(
///     parse_timezone_offset("-07:00"),
///     TimeZoneOffset::new_opt(-7, 0)
/// );
/// ```
///
/// [whatwg-html-tzoffset]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#time-zones
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-time-zone-offset-string
#[inline]
pub fn parse_timezone_offset(s: &str) -> Option<TimeZoneOffset> {
	parse_format(s, parse_timezone_offset_component)
}

/// Low-level function for parsing an individual timezone offset component
///
/// > **Note**:
/// > This function exposes a lower-level API than [`parse_timezone_offset`].
/// > More than likely, you will want to use [`parse_timezone_offset`] instead.
///
/// # Examples
/// ```
/// use whatwg_datetime::{parse_timezone_offset_component, TimeZoneOffset};
///
/// let mut position = 0usize;
/// let date = parse_timezone_offset_component("-07:00", &mut position);
///
/// assert_eq!(date, TimeZoneOffset::new_opt(-7, 0));
/// ```
pub fn parse_timezone_offset_component(s: &str, position: &mut usize) -> Option<TimeZoneOffset> {
	let char_at = s.chars().nth(*position);

	let mut minutes = 0i32;
	let mut hours = 0i32;

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
				hours = collected.parse::<i32>().unwrap();
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

				minutes = parsed_mins.parse::<i32>().unwrap();
			} else if collected_len == 4 {
				let (hour_str, min_str) = collected.split_at(2);
				hours = hour_str.parse::<i32>().unwrap();
				minutes = min_str.parse::<i32>().unwrap();
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
	fn parse_timezone_offset_fails_not_colon() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01/", &mut position);

		assert_eq!(parsed, None);
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
