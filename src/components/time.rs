use crate::parse_format;
use crate::tokens::{TOKEN_COLON, TOKEN_DOT};
use crate::utils::{collect_ascii_digits, is_valid_hour, is_valid_min_or_sec};
use chrono::NaiveTime;
use whatwg_infra::collect_codepoints;

/// Parse a specific time containing an hour, minute, and optionally a second,
/// and a fraction of a second
///
/// This follows the rules for [parsing a time string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.4 Times][whatwg-html-time].
///
/// # Examples
/// ```
/// use chrono::NaiveTime;
/// use whatwg_datetime::parse_time;
///
/// // parse a local datetime with hours and minutes
/// assert_eq!(parse_time("14:59"), NaiveTime::from_hms_opt(14, 59, 0));
///
/// // parse a local datetime with hours, minutes, and seconds
/// assert_eq!(parse_time("14:59:39"), NaiveTime::from_hms_opt(14, 59, 39));
///
/// // parse a local datetime with hours, minutes, seconds, and milliseconds
/// assert_eq!(parse_time("14:59:39.929"), NaiveTime::from_hms_milli_opt(14, 59, 39, 929));
/// ```
///
/// [whatwg-html-time]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#times
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-time-string
#[inline]
pub fn parse_time(s: &str) -> Option<NaiveTime> {
	parse_format(s, parse_time_component)
}

/// Low-level function for parsing an individual time component at a given position
///
/// This follows the rules for [parsing a time component][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.4 Times][whatwg-html-time].
///
/// > **Note**:
/// > This function exposes a lower-level API than [`parse_time`]. More than likely,
/// > you will want to use [`parse_time`] instead.
///
/// # Examples
/// ```
/// use chrono::NaiveTime;
/// use whatwg_datetime::parse_time_component;
///
/// let mut position = 0usize;
/// let date = parse_time_component("14:59", &mut position);
///
/// assert_eq!(date, NaiveTime::from_hms_opt(14, 59, 0));
/// ```
///
/// [whatwg-html-time]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#times
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-time-component
pub fn parse_time_component(s: &str, position: &mut usize) -> Option<NaiveTime> {
	let parsed_hour = collect_ascii_digits(s, position);
	if parsed_hour.len() != 2 {
		return None;
	}

	let hour = parsed_hour.parse::<u32>().ok()?;
	if !is_valid_hour(&hour) {
		return None;
	}

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_COLON) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_minute = collect_ascii_digits(s, position);
	if parsed_minute.len() != 2 {
		return None;
	}
	let minute = parsed_minute.parse::<u32>().ok()?;
	if !is_valid_min_or_sec(&minute) {
		return None;
	}

	let mut seconds = 0u32;
	let mut milliseconds = 0u32;
	if *position < s.len() && s.chars().nth(*position) == Some(TOKEN_COLON) {
		*position += 1;

		if *position >= s.len() {
			return None;
		}

		let parsed_second =
			collect_codepoints(s, position, |c| c.is_ascii_digit() || c == TOKEN_DOT);
		let parsed_second_len = parsed_second.len();
		if parsed_second_len == 3
			|| (parsed_second_len > 3
				&& parsed_second.chars().nth(2) != Some(TOKEN_DOT))
			|| has_at_least_n_instances(s, TOKEN_DOT, 2)
		{
			return None;
		}

		let (parsed_seconds, parsed_milliseconds) =
			parse_seconds_milliseconds(&parsed_second);
		seconds = parsed_seconds;
		milliseconds = parsed_milliseconds;
		if !is_valid_min_or_sec(&seconds) {
			return None;
		}
	}

	NaiveTime::from_hms_milli_opt(hour, minute, seconds, milliseconds)
}

fn has_at_least_n_instances(s: &str, c: char, n: usize) -> bool {
	let mut count = 0usize;
	for ch in s.chars() {
		if ch == c {
			count += 1usize;
			if count >= n {
				return true;
			}
		}
	}
	false
}

fn parse_seconds_milliseconds(s: &str) -> (u32, u32) {
	let parts: Vec<&str> = s.split(TOKEN_DOT).collect();
	let seconds = parts.first().unwrap_or(&"0").parse().unwrap_or(0);
	let milliseconds = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);

	(seconds, milliseconds)
}

#[cfg(test)]
mod tests {
	use super::{parse_time, parse_time_component, NaiveTime};

	#[test]
	fn test_parse_time_succeeds_hm() {
		assert_eq!(
			parse_time("12:31"),
			NaiveTime::from_hms_milli_opt(12, 31, 0, 0)
		);
	}

	#[test]
	fn test_parse_time_succeeds_hms() {
		assert_eq!(
			parse_time("12:31:59"),
			NaiveTime::from_hms_milli_opt(12, 31, 59, 0)
		);
	}

	#[test]
	fn test_parse_time_succeeds_hms_fractional_seconds() {
		assert_eq!(
			parse_time("14:54:39.929"),
			NaiveTime::from_hms_milli_opt(14, 54, 39, 929)
		);
	}

	#[test]
	fn test_parse_time_fails_multiple_decimals() {
		assert_eq!(parse_time("12:31:59...29"), None);
	}

	#[test]
	fn test_parse_time_fails_hour_length() {
		assert_eq!(parse_time("123:31:59"), None);
	}

	#[test]
	fn test_parse_time_fails_hour_value_upper_bound() {
		assert_eq!(parse_time("24:31:59"), None);
	}

	#[test]
	fn test_parse_time_fails_delimiter() {
		assert_eq!(parse_time("12-31-59"), None);
	}

	#[test]
	fn test_parse_time_fails_minute_length() {
		assert_eq!(parse_time("12:311:59"), None);
	}

	#[test]
	fn test_parse_time_fails_minute_value_upper_bound() {
		assert_eq!(parse_time("12:79:59"), None);
	}

	#[test]
	fn test_parse_time_fails_seconds_length() {
		assert_eq!(parse_time("12:31:591"), None);
	}

	#[test]
	fn test_parse_time_fails_seconds_value_upper_bound() {
		assert_eq!(parse_time("12:31:79"), None);
	}

	#[test]
	fn test_parse_time_component() {
		let mut position = 0usize;
		let parsed = parse_time_component("12:31:59", &mut position);

		assert_eq!(parsed, NaiveTime::from_hms_milli_opt(12, 31, 59, 0));
	}
}
