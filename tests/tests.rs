use Generator_SH_KH_DPH::*; 
use time::{Date, Month};

#[test]
fn test_one_month_earlier_normal_case() {
    let d = Date::from_calendar_date(2025, Month::July, 11).unwrap();
    let prev = one_month_earlier(d);
    assert_eq!(prev, Date::from_calendar_date(2025, Month::June, 11).unwrap());
}

#[test]
fn test_one_month_earlier_january_to_december() {
    let d = Date::from_calendar_date(2025, Month::January, 15).unwrap();
    let prev = one_month_earlier(d);
    assert_eq!(prev, Date::from_calendar_date(2024, Month::December, 15).unwrap());
}

#[test]
fn test_one_month_earlier_clamp_day() {
    let d = Date::from_calendar_date(2025, Month::March, 31).unwrap();
    let prev = one_month_earlier(d);
    assert_eq!(prev, Date::from_calendar_date(2025, Month::February, 28).unwrap());
}
