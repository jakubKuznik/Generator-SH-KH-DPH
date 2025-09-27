use reqwest::blocking::Client;
use serde::Deserialize;
use time::{Date, Month, OffsetDateTime};

#[derive(Debug)]
pub struct Config {
    pub hodnota_plneni_czk: i32,
    pub datum_podpisu: Date,
    pub datum_za_obdobi: Date,
    pub kurz_eur: f32,
    pub kurz_usd: f32,
}

// Make functions public so tests and main.rs can use them
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
