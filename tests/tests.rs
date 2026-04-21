use generator_sh_kh_dph::date_func::one_month_earlier;
use generator_sh_kh_dph::rates::get_monthly_average_czk_rate;
use generator_sh_kh_dph::{dph::generate_dph, kh::generate_kh, Config};
use std::fs::{remove_file, File};
use std::io::Read;
use time::{Date, Month};
use xmltree::Element;

#[test]
fn test_one_month_earlier_normal_case() {
    let d = Date::from_calendar_date(2025, Month::July, 11).unwrap();
    let prev = one_month_earlier(d);
    assert_eq!(
        prev,
        Date::from_calendar_date(2025, Month::June, 11).unwrap()
    );
}

#[test]
fn test_one_month_earlier_january_to_december() {
    let d = Date::from_calendar_date(2025, Month::January, 15).unwrap();
    let prev = one_month_earlier(d);
    assert_eq!(
        prev,
        Date::from_calendar_date(2024, Month::December, 15).unwrap()
    );
}

/// --- Currency API integration tests ---

#[test]
fn test_same_month_day_vs_first_day() {
    let d1 = get_monthly_average_czk_rate(2025, 3, "EUR").unwrap();
    let d2 = get_monthly_average_czk_rate(2025, 3, "EUR").unwrap();
    assert!(
        (d1 - d2).abs() < 1e-6,
        "March 15th and March 1st should match"
    );
}

#[test]
fn test_known_eur_rate_march_2025() {
    let avg = get_monthly_average_czk_rate(2025, 3, "EUR").unwrap();
    // Expected ~25.003 (check CNB)
    assert!(
        (avg - 25.003).abs() < 0.01,
        "Expected ≈ 25.003, got {}",
        avg
    );
}

#[test]
fn test_known_eur_rate_november_2022() {
    let avg = get_monthly_average_czk_rate(2022, 11, "EUR").unwrap();
    assert!(
        (avg - 24.367).abs() < 0.01,
        "Expected ≈ 24.367, got {}",
        avg
    );
}

#[test]
fn test_known_usd_rate_august_2025() {
    let avg = get_monthly_average_czk_rate(2025, 8, "USD").unwrap();
    assert!(
        (avg - 21.079).abs() < 0.01,
        "Expected ≈ 21.079, got {}",
        avg
    );
}

#[test]
fn test_known_usd_rate_january_2021() {
    let avg = get_monthly_average_czk_rate(2021, 1, "USD").unwrap();
    assert!(
        (avg - 21.479).abs() < 0.01,
        "Expected ≈ 21.479, got {}",
        avg
    );
}

#[test]
fn test_future_year_should_fail() {
    let res = get_monthly_average_czk_rate(2055, 1, "EUR");
    assert!(
        res.is_err(),
        "Expected error for year 2055, but got {:?}",
        res
    );
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
        prijata_zdanitelna_plneni_nad_limit_czk: 0.0,
        dph_prijata_zdanitelna_plneni_nad_limit_czk: 0.0,
        prijata_zdanitelna_plneni_nad_limit_dic: None,
        prijata_zdanitelna_plneni_nad_limit_doklad: None,
        prijata_zdanitelna_plneni_nad_limit_dppd: None,
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
    let vetad = dphdp3.get_child("VetaD").unwrap();
    let veta4 = dphdp3.get_child("Veta4").unwrap();
    let veta6 = dphdp3.get_child("Veta6").unwrap();

    assert_eq!(
        vetad.attributes.get("c_okec").map(String::as_str),
        Some("622000")
    );
    assert_eq!(
        veta4.attributes.get("pln23").map(String::as_str),
        Some("4111")
    );
    assert_eq!(
        veta4.attributes.get("odp_tuz23_nar").map(String::as_str),
        Some("863")
    );
    assert_eq!(
        veta6.attributes.get("dano_no").map(String::as_str),
        Some("863")
    );

    let _ = remove_file("DPH-2026-01.xml");
}

#[test]
fn test_tuzemske_plneni_nad_limit_goes_to_kh_control_total_and_dph_row_40_totals() {
    let config = Config {
        datum_podpisu: Date::from_calendar_date(2026, Month::April, 16).unwrap(),
        datum_za_obdobi: Date::from_calendar_date(2026, Month::March, 16).unwrap(),
        kurz_eur: 24.279,
        kurz_usd: 20.684,
        hodnota_plneni_eur: 0.0,
        hodnota_plneni_czk: 0,
        prijata_zdanitelna_plneni_czk: 2045.45,
        dph_prijata_zdanitelna_plneni_czk: 429.5445,
        prijata_zdanitelna_plneni_nad_limit_czk: 19865.29,
        dph_prijata_zdanitelna_plneni_nad_limit_czk: 4172.0,
        prijata_zdanitelna_plneni_nad_limit_dic: Some("27082440".to_string()),
        prijata_zdanitelna_plneni_nad_limit_doklad: Some("589791468".to_string()),
        prijata_zdanitelna_plneni_nad_limit_dppd: Some(
            Date::from_calendar_date(2026, Month::March, 25).unwrap(),
        ),
        prijeti_sluzeb_v_jinem_state_usd: 0.0,
        prijeti_sluzeb_v_jinem_state_czk: 0,
        dph_prijeti_sluzeb_v_jinem_state_czk: 0,
    };

    generate_kh(&config);
    generate_dph(&config);

    let mut kh_xml = String::new();
    File::open("KH-2026-03.xml")
        .unwrap()
        .read_to_string(&mut kh_xml)
        .unwrap();
    let kh_root = Element::parse(kh_xml.as_bytes()).unwrap();
    let dphkh1 = kh_root.get_child("DPHKH1").unwrap();
    let vetab2 = dphkh1.get_child("VetaB2").unwrap();
    let vetac = dphkh1.get_child("VetaC").unwrap();

    assert_eq!(
        vetab2.attributes.get("c_evid_dd").map(String::as_str),
        Some("589791468")
    );
    assert_eq!(
        vetab2.attributes.get("dic_dod").map(String::as_str),
        Some("27082440")
    );
    assert_eq!(
        vetab2.attributes.get("dppd").map(String::as_str),
        Some("25.03.2026")
    );
    assert_eq!(
        vetab2.attributes.get("zakl_dane1").map(String::as_str),
        Some("19865.29")
    );
    assert_eq!(
        vetab2.attributes.get("dan1").map(String::as_str),
        Some("4171.71")
    );
    assert_eq!(
        vetab2.attributes.get("pomer").map(String::as_str),
        Some("N")
    );
    assert_eq!(
        vetab2.attributes.get("zdph_44").map(String::as_str),
        Some("N")
    );
    assert_eq!(
        vetac.attributes.get("pln23").map(String::as_str),
        Some("21910.74")
    );

    let mut dph_xml = String::new();
    File::open("DPH-2026-03.xml")
        .unwrap()
        .read_to_string(&mut dph_xml)
        .unwrap();
    let dph_root = Element::parse(dph_xml.as_bytes()).unwrap();
    let dphdp3 = dph_root.get_child("DPHDP3").unwrap();
    let veta4 = dphdp3.get_child("Veta4").unwrap();
    let veta6 = dphdp3.get_child("Veta6").unwrap();

    assert_eq!(
        veta4.attributes.get("pln23").map(String::as_str),
        Some("21911")
    );
    assert_eq!(
        veta4.attributes.get("odp_tuz23_nar").map(String::as_str),
        Some("4601")
    );
    assert_eq!(
        veta6.attributes.get("dano_no").map(String::as_str),
        Some("4601")
    );

    let _ = remove_file("KH-2026-03.xml");
    let _ = remove_file("DPH-2026-03.xml");
}
