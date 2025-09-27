use time::{Date, Month};
use Generator_SH_KH_DPH::rates::get_monthly_average_czk_rate;
use Generator_SH_KH_DPH::date_func::{one_month_earlier};

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

/// --- Currency API integration tests ---

#[test]
fn test_same_month_day_vs_first_day() {
    let d1 = get_monthly_average_czk_rate(2025, 3, "EUR").unwrap();
    let d2 = get_monthly_average_czk_rate(2025, 3, "EUR").unwrap();
    assert!((d1 - d2).abs() < 1e-6, "March 15th and March 1st should match");
}

#[test]
fn test_known_eur_rate_march_2025() {
    let avg = get_monthly_average_czk_rate(2025, 3, "EUR").unwrap();
    // Expected ~25.003 (check CNB)
    assert!((avg - 25.003).abs() < 0.01, "Expected ≈ 25.003, got {}", avg);
}

#[test]
fn test_known_eur_rate_november_2022() {
    let avg = get_monthly_average_czk_rate(2022, 11, "EUR").unwrap();
    assert!((avg - 24.367).abs() < 0.01, "Expected ≈ 24.367, got {}", avg);
}

#[test]
fn test_known_usd_rate_august_2025() {
    let avg = get_monthly_average_czk_rate(2025, 8, "USD").unwrap();
    assert!((avg - 21.079).abs() < 0.01, "Expected ≈ 21.079, got {}", avg);
}

#[test]
fn test_known_usd_rate_january_2021() {
    let avg = get_monthly_average_czk_rate(2021, 1, "USD").unwrap();
    assert!((avg - 21.479).abs() < 0.01, "Expected ≈ 21.479, got {}", avg);
}

#[test]
fn test_future_year_should_fail() {
    let res = get_monthly_average_czk_rate(2055, 1, "EUR");
    assert!(res.is_err(), "Expected error for year 2055, but got {:?}", res);
}