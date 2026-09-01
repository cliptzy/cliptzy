/// Returns the number of days in a given month (1-12) for the given year.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Adds `days` to a YYYYMMDD date string, returning the new date or None on invalid input.
pub fn add_days_yyyymmdd(date: &str, days: u32) -> Option<String> {
    if date.len() != 8 {
        return None;
    }
    let mut year: i32 = date[0..4].parse().ok()?;
    let mut month: u32 = date[4..6].parse().ok()?;
    let mut day: u32 = date[6..8].parse().ok()?;
    let mut remaining = days;

    while remaining > 0 {
        let dim = days_in_month(year, month);
        let days_left_in_month = dim.saturating_sub(day);
        if remaining <= days_left_in_month {
            day += remaining;
            remaining = 0;
        } else {
            remaining -= days_left_in_month + 1;
            day = 1;
            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }
        }
    }

    Some(format!("{:04}{:02}{:02}", year, month, day))
}

/// Returns true when `entry_date` falls within `max_days_after` days of `main_date` (YYYYMMDD).
pub fn is_upload_within_reaction_window(
    entry_date: &str,
    main_date: &str,
    max_days_after: u32,
) -> bool {
    if entry_date.len() != 8 || main_date.len() != 8 {
        return false;
    }
    let max_date =
        add_days_yyyymmdd(main_date, max_days_after).unwrap_or_else(|| "99999999".to_string());
    entry_date >= main_date && entry_date <= max_date.as_str()
}
