use crate::{
	utils::{collect_ascii_digits, week_number_of_year},
	TOKEN_ABBR_WEEK, TOKEN_DATETIME_SEPARATOR,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWeek {
	pub(crate) year: i32,
	pub(crate) week: u8,
}

pub fn parse_week(input: &str) -> Option<YearWeek> {
	// Step 1, 2
	let mut position = 0usize;

	// Step 3, 4
	let year_string = collect_ascii_digits(input, &mut position);
	let year = year_string.parse::<i32>().unwrap();
	if year <= 0 {
		return None;
	}

	// Step 5
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		position += 1;
	}

	// Step 6
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_ABBR_WEEK) {
		return None;
	} else {
		position += 1;
	}

	// Step 7
	let parsed_week = collect_ascii_digits(input, &mut position);
	if parsed_week.len() != 2 {
		return None;
	}

	let week = parsed_week.parse::<u8>().unwrap();
	let max_weeks = week_number_of_year(year)?;
	if week < 1 || week > max_weeks {
		return None;
	}

	Some(YearWeek { year, week })
}

#[cfg(test)]
mod tests {
	use super::{parse_week, YearWeek};

	#[test]
	fn test_parse_week_string() {
		assert_eq!(
			parse_week("2004-W53"),
			Some(YearWeek {
				year: 2004,
				week: 53
			})
		);
	}
}
