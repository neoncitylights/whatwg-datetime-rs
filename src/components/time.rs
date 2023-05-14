use chrono::NaiveTime;
use whatwg_infra::collect_codepoints;
use crate::utils::{collect_ascii_digits, is_valid_hour, is_valid_min_or_sec};

pub fn parse_time_component(s: &str, position: &mut usize) -> Option<NaiveTime> {
	let parsed_hour = collect_ascii_digits(s, position);
	if parsed_hour.len() != 2 {
		return None;
	}

	let hour = parsed_hour.parse::<u8>().ok()?;
	if !is_valid_hour(&hour) {
		return None;
	}

	if *position > s.len() || s.chars().nth(*position) != Some(':') {
		return None;
	} else {
		*position += 1;
	}

	let parsed_minute = collect_ascii_digits(s, position);
	if parsed_minute.len() != 2 {
		return None;
	}
	let minute = parsed_minute.parse::<u8>().ok()?;
	if !is_valid_min_or_sec(&minute) {
		return None;
	}

	let mut second = 0u8;
	if *position < s.len() && s.chars().nth(*position) == Some(':') {
		*position += 1;

		if *position >= s.len() {
			return None;
		}

		let parsed_second =
			collect_codepoints(s, position, |c| c.is_ascii_digit() || c == '.');
		let parsed_second_len = parsed_second.len();
		if parsed_second_len == 3
			|| (parsed_second_len > 3 && parsed_second.chars().nth(3) != Some('.'))
			|| parsed_second.chars().any(|c| c == '.')
		{
			return None;
		}

		second = parsed_second.parse::<u8>().ok()?;
		if !is_valid_min_or_sec(&second) {
			return None;
		}
	}

	NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32)
}
