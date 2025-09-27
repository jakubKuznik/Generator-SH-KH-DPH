use time::{Date, Month, OffsetDateTime, Duration};

pub fn get_today() -> Date {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    now.date()
}

pub fn one_month_earlier(d: Date) -> Date {
    let y = d.year();
    let m = d.month();
    let day = d.day();

    let (new_y, new_m) = if m == Month::January {
        (y - 1, Month::December)
    } else {
        (y, m.previous())
    };

    let first_of_next = if new_m == Month::December {
        Date::from_calendar_date(new_y + 1, Month::January, 1).unwrap()
    } else {
        Date::from_calendar_date(new_y, new_m.next(), 1).unwrap()
    };
    let last_of_new = first_of_next - time::Duration::days(1);

    let safe_day = day.min(last_of_new.day());
    Date::from_calendar_date(new_y, new_m, safe_day).unwrap()
}

/// Vrátí první den v měsíci ve formátu dd.mm.yyyy
pub fn first_month_day(year: i32, month: Month) -> String {
    let first_day = Date::from_calendar_date(year, month, 1).unwrap();
    format!("{:02}.{:02}.{}", first_day.day(), month as u8, year)
}

/// Vrátí poslední den v měsíci ve formátu dd.mm.yyyy
pub fn last_month_day(year: i32, month: Month) -> String {
    let first_of_next = if month == Month::December {
        Date::from_calendar_date(year + 1, Month::January, 1).unwrap()
    } else {
        Date::from_calendar_date(year, month.next(), 1).unwrap()
    };
    let last_day = first_of_next - Duration::days(1);
    format!("{:02}.{:02}.{}", last_day.day(), month as u8, year)
}