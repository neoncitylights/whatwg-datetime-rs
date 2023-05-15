use crate::{parse_date_component, parse_time_component};
use chrono::NaiveDateTime;

pub fn parse_local_datetime(s: &str) -> Option<NaiveDateTime> {
	let mut position = 0usize;
	let date = parse_date_component(s, &mut position)?;

	let last_char = s.chars().nth(position);
	if position > s.len() || !matches!(last_char, Some('T') | Some(' ')) {
		return None;
	} else {
		position += 1;
	}

	let time = parse_time_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(NaiveDateTime::new(date, time))
}

#[cfg(test)]
mod tests {
	use super::parse_local_datetime;
	use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

	#[test]
	pub fn test_parse_local_datetime() {
		let parsed = parse_local_datetime("2004-12-31T12:31:59");

		assert_eq!(
			parsed,
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
				NaiveTime::from_hms_opt(12, 31, 59).unwrap(),
			))
		);
	}
}
