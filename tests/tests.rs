use time::{Date, Month};
use generator_sh_kh_dph::rates::get_monthly_average_czk_rate;
use generator_sh_kh_dph::date_func::{one_month_earlier};
use generator_sh_kh_dph::{Config, dph::generate_dph};
use std::fs::{remove_file, File};
use std::io::Read;
use xmltree::Element;

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

#[test]
fn test_dph_rounding_bug_4110_74() {
    let config = Config {
        datum_podpisu: Date::from_calendar_date(2026, Month::February, 16).unwrap(),
        datum_za_obdobi: Date::from_calendar_date(2026, Month::January, 16).unwrap(),
        kurz_eur: 24.279,
        kurz_usd: 20.684,
        hodnota_plneni_eur: 2905.54,
        hodnota_plneni_czk: 70544,
        prijata_zdanitelna_plneni_czk: 4110.74,
        dph_prijata_zdanitelna_plneni_czk: 863.2554,
        prijeti_sluzeb_v_jinem_state_usd: 0.0,
        prijeti_sluzeb_v_jinem_state_czk: 0,
        dph_prijeti_sluzeb_v_jinem_state_czk: 0,
    };

    generate_dph(&config);

    let mut xml_str = String::new();
    File::open("DPH-2026-01.xml")
        .unwrap()
        .read_to_string(&mut xml_str)
        .unwrap();
    let root = Element::parse(xml_str.as_bytes()).unwrap();
    let dphdp3 = root.get_child("DPHDP3").unwrap();
    let veta4 = dphdp3.get_child("Veta4").unwrap();
    let veta6 = dphdp3.get_child("Veta6").unwrap();

    assert_eq!(veta4.attributes.get("pln23").map(String::as_str), Some("4111"));
    assert_eq!(veta4.attributes.get("odp_tuz23_nar").map(String::as_str), Some("863"));
    assert_eq!(veta6.attributes.get("dano_no").map(String::as_str), Some("863"));

    let _ = remove_file("DPH-2026-01.xml");
}
