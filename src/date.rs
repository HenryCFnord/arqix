//! Calendar-date validation, consolidated (refactor slice R5). Two
//! deliberately different contracts live here side by side:
//! `is_calendar_date` is the checkers' oracle-faithful permissive parse
//! (unpadded components accepted), `valid_iso_date` is the strict
//! YYYY-MM-DD gate for CLI input. Unifying them would be a
//! behaviour-visible change and needs its own slice.

pub(crate) fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub(crate) fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub(crate) fn is_calendar_date(value: &str) -> bool {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    let (year, month, day) = match (
        parts[0].parse::<i64>(),
        parts[1].parse::<u32>(),
        parts[2].parse::<u32>(),
    ) {
        (Ok(y), Ok(m), Ok(d)) => (y, m, d),
        _ => return false,
    };
    (1..=9999).contains(&year)
        && (1..=12).contains(&month)
        && (1..=days_in_month(year, month)).contains(&day)
}

/// A real ISO-8601 calendar date in `YYYY-MM-DD` — shape and calendar both
/// (REQ-01-01-06-01), so finalise can never stamp a value its own
/// frontmatter rules reject.
// arqix:implements REQ-01-01-06-01
pub(crate) fn valid_iso_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits = |r: std::ops::Range<usize>| -> Option<u32> {
        s.get(r)
            .filter(|p| p.bytes().all(|b| b.is_ascii_digit()))?
            .parse()
            .ok()
    };
    let (Some(year), Some(month), Some(day)) = (digits(0..4), digits(5..7), digits(8..10)) else {
        return false;
    };
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}
