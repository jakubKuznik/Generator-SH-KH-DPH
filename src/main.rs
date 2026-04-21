use generator_sh_kh_dph::date_func::{get_today, one_month_earlier};
use generator_sh_kh_dph::dph::generate_dph;
use generator_sh_kh_dph::kh::generate_kh;
use generator_sh_kh_dph::rates::get_monthly_average_czk_rate;
use generator_sh_kh_dph::sh::generate_sh;
use generator_sh_kh_dph::Config;
use std::env;
use std::process::exit;
use time::format_description;
use time::Date;

fn usage() -> &'static str {
    "Usage: app --hodnota-plneni <EUR> --prijata-zdanitelna-plneni <CZK> (--prijeti-sluzeb-v-jinem-state <USD> | --prijeti-sluzeb-v-jinem-state-czk <CZK>) [--prijata-zdanitelna-plneni-nad-10000 <CZK> dic <DIC> danovydoklad <TEXT> datum <D.M.YYYY>] [--datum-za-obdobi <YYYY-MM-DD>]\n
    Options:\n
        -h, --help Show this help\n
        --datum-za-obdobi <YYYY-MM-DD>\n
            nebo alias --datum <YYYY-MM-DD>\n
            Přepíše výchozí období (jinak se bere minulý měsíc).\n
        --hodnota-plneni <EUR>\n
        --prijata-zdanitelna-plneni <CZK>\n
            maximalne 10 000czk na jednotlivé položky\n
            Částka BEZ DPH!\n
            podnajem kancelare, nakup mobilu\n
        --prijata-zdanitelna-plneni-nad-10000 <CZK>\n
            Souhrn tuzemských daňových dokladů nad 10 000 Kč včetně DPH.\n
            Částka BEZ DPH, DPH se dopočítá automaticky sazbou 21 %.\n
            Vyžaduje také: dic <DIC> danovydoklad <TEXT> datum <D.M.YYYY> pro KH B.2.\n
        --prijeti-sluzeb-v-jinem-state <USD>\n
            Částka BEZ DPH!\n
            licence chatGPT fakturovaná v USD,\n
        --prijeti-sluzeb-v-jinem-state-czk <CZK>\n
            Částka BEZ DPH!\n
            licence chatGPT fakturovaná rovnou v CZK,
        "
}

fn parse_iso_date(input: &str) -> Date {
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    Date::parse(input, &format).unwrap_or_else(|e| {
        panic!("Invalid date '{}', expected YYYY-MM-DD: {}", input, e);
    })
}

fn parse_czech_date(input: &str) -> Date {
    let parts: Vec<&str> = input.split('.').filter(|part| !part.is_empty()).collect();
    if parts.len() != 3 {
        panic!("Invalid date '{}', expected D.M.YYYY", input);
    }

    let day: u8 = parts[0]
        .parse()
        .unwrap_or_else(|e| panic!("Invalid day in '{}': {}", input, e));
    let month_number: u8 = parts[1]
        .parse()
        .unwrap_or_else(|e| panic!("Invalid month in '{}': {}", input, e));
    let year: i32 = parts[2]
        .parse()
        .unwrap_or_else(|e| panic!("Invalid year in '{}': {}", input, e));
    let month = time::Month::try_from(month_number)
        .unwrap_or_else(|e| panic!("Invalid month in '{}': {}", input, e));

    Date::from_calendar_date(year, month, day)
        .unwrap_or_else(|e| panic!("Invalid date '{}': {}", input, e))
}

fn dic_without_country_code(dic: &str) -> String {
    dic.trim()
        .strip_prefix("CZ")
        .or_else(|| dic.trim().strip_prefix("cz"))
        .unwrap_or(dic.trim())
        .to_string()
}

fn amount_czk_and_dph(amount_czk: f64) -> (i64, i64) {
    (amount_czk.ceil() as i64, (amount_czk * 0.21).ceil() as i64)
}

fn parse_args() -> Config {
    let mut args = env::args().skip(1);
    if args.len() == 0 {
        println!("{}", usage());
        exit(0);
    }

    let today = get_today();
    let mut datum_za_obdobi = one_month_earlier(today);
    let mut hodnota_plneni_eur: Option<f64> = None;
    let mut prijata_zdanitelna_plneni_czk: Option<f64> = None;
    let mut prijata_zdanitelna_plneni_nad_limit_czk: Option<f64> = None;
    let mut prijata_zdanitelna_plneni_nad_limit_dic: Option<String> = None;
    let mut prijata_zdanitelna_plneni_nad_limit_doklad: Option<String> = None;
    let mut prijata_zdanitelna_plneni_nad_limit_dppd: Option<Date> = None;
    let mut prijeti_sluzeb_v_jinem_state_usd: Option<f64> = None;
    let mut prijeti_sluzeb_v_jinem_state_czk: Option<f64> = None;

    while let Some(arg) = args.next() {
        match arg.to_lowercase().as_str() {
            "-h" | "--help" => {
                println!("{}", usage());
                exit(0);
            }
            "--datum-za-obdobi" | "--datum" => {
                if let Some(val) = args.next() {
                    datum_za_obdobi = parse_iso_date(&val);
                } else {
                    panic!("Error: --datum-za-obdobi requires value in format YYYY-MM-DD");
                }
            }
            "--hodnota-plneni" => {
                if let Some(val) = args.next() {
                    hodnota_plneni_eur = Some(val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --hodnota-plneni '{}': {}", val, e);
                    }));
                } else {
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            "--prijata-zdanitelna-plneni" => {
                if let Some(val) = args.next() {
                    prijata_zdanitelna_plneni_czk = Some(val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --prijata-zdanitelna-plneni '{}': {}", val, e);
                    }));
                } else {
                    panic!("Error: --prijata-zdanitelna-plneni requires a positive int value");
                }
            }
            "--prijata-zdanitelna-plneni-nad-10000" => {
                if let Some(val) = args.next() {
                    prijata_zdanitelna_plneni_nad_limit_czk =
                        Some(val.parse().unwrap_or_else(|e| {
                            panic!(
                                "Invalid --prijata-zdanitelna-plneni-nad-10000 '{}': {}",
                                val, e
                            );
                        }));
                } else {
                    panic!("Error: --prijata-zdanitelna-plneni-nad-10000 requires a value");
                }
            }
            "--prijeti-sluzeb-v-jinem-state" => {
                if let Some(val) = args.next() {
                    prijeti_sluzeb_v_jinem_state_usd = Some(val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --prijeti-sluzeb-v-jinem-state '{}': {}", val, e);
                    }));
                } else {
                    panic!("Error: --prijeti-sluzeb-v-jinem-state requires a positive int value");
                }
            }
            "--prijeti-sluzeb-v-jinem-state-czk" => {
                if let Some(val) = args.next() {
                    prijeti_sluzeb_v_jinem_state_czk = Some(val.parse().unwrap_or_else(|e| {
                        panic!(
                            "Invalid --prijeti-sluzeb-v-jinem-state-czk '{}': {}",
                            val, e
                        );
                    }));
                } else {
                    panic!(
                        "Error: --prijeti-sluzeb-v-jinem-state-czk requires a positive int value"
                    );
                }
            }
            "dic" => {
                if let Some(val) = args.next() {
                    prijata_zdanitelna_plneni_nad_limit_dic = Some(dic_without_country_code(&val));
                } else {
                    panic!("Error: dic requires a value");
                }
            }
            "danovydoklad" => {
                if let Some(val) = args.next() {
                    prijata_zdanitelna_plneni_nad_limit_doklad = Some(val);
                } else {
                    panic!("Error: danovydoklad requires a value");
                }
            }
            "datum" => {
                if let Some(val) = args.next() {
                    prijata_zdanitelna_plneni_nad_limit_dppd = Some(parse_czech_date(&val));
                } else {
                    panic!("Error: datum requires a value in D.M.YYYY format");
                }
            }
            _ => println!("Unknown arg: {}", arg),
        }
    }

    let hodnota_plneni_eur = hodnota_plneni_eur.unwrap_or_else(|| {
        panic!("Error: --hodnota-plneni requires a positive int value");
    });
    let prijata_zdanitelna_plneni_czk = prijata_zdanitelna_plneni_czk.unwrap_or_else(|| {
        panic!("Error: --prijata-zdanitelna-plneni requires a positive int value");
    });
    let prijata_zdanitelna_plneni_nad_limit_czk =
        prijata_zdanitelna_plneni_nad_limit_czk.unwrap_or(0.0);
    if prijata_zdanitelna_plneni_nad_limit_czk > 0.0 {
        if prijata_zdanitelna_plneni_nad_limit_dic.is_none() {
            panic!("Error: dic is required when --prijata-zdanitelna-plneni-nad-10000 is used");
        }
        if prijata_zdanitelna_plneni_nad_limit_doklad.is_none() {
            panic!(
                "Error: danovydoklad is required when --prijata-zdanitelna-plneni-nad-10000 is used"
            );
        }
        if prijata_zdanitelna_plneni_nad_limit_dppd.is_none() {
            panic!("Error: datum is required when --prijata-zdanitelna-plneni-nad-10000 is used");
        }
    }

    if prijeti_sluzeb_v_jinem_state_usd.is_some() && prijeti_sluzeb_v_jinem_state_czk.is_some() {
        panic!(
            "Error: use either --prijeti-sluzeb-v-jinem-state or --prijeti-sluzeb-v-jinem-state-czk, not both"
        );
    }
    if prijeti_sluzeb_v_jinem_state_usd.is_none() && prijeti_sluzeb_v_jinem_state_czk.is_none() {
        panic!(
            "Error: --prijeti-sluzeb-v-jinem-state or --prijeti-sluzeb-v-jinem-state-czk requires a positive int value"
        );
    }

    if hodnota_plneni_eur < 0.0 {
        panic!(
            "Error: --hodnota-plneni requires a positive int value, it has: {}",
            hodnota_plneni_eur
        );
    }
    if prijata_zdanitelna_plneni_czk < 0.0 {
        panic!(
            "Error: --prijata-zdanitelna-plneni a positive int value, it has: {}",
            prijata_zdanitelna_plneni_czk
        );
    }
    if prijata_zdanitelna_plneni_nad_limit_czk < 0.0 {
        panic!(
            "Error: --prijata-zdanitelna-plneni-nad-10000 requires a positive value, it has: {}",
            prijata_zdanitelna_plneni_nad_limit_czk
        );
    }
    if let Some(usd) = prijeti_sluzeb_v_jinem_state_usd {
        if usd < 0.0 {
            panic!(
                "Error: --prijeti_sluzeb_v_jinem_state_usd a positive int value, it has: {}",
                usd
            );
        }
    }
    if let Some(czk) = prijeti_sluzeb_v_jinem_state_czk {
        if czk < 0.0 {
            panic!(
                "Error: --prijeti_sluzeb_v_jinem_state_czk a positive int value, it has: {}",
                czk
            );
        }
    }

    let kurz_eur =
        get_monthly_average_czk_rate(datum_za_obdobi.year(), datum_za_obdobi.month() as u8, "EUR")
            .unwrap();

    let (
        kurz_usd,
        prijeti_sluzeb_v_jinem_state_usd,
        prijeti_sluzeb_v_jinem_state_czk,
        dph_prijeti_sluzeb_v_jinem_state_czk,
    ) = if let Some(amount_usd) = prijeti_sluzeb_v_jinem_state_usd {
        let kurz_usd = get_monthly_average_czk_rate(
            datum_za_obdobi.year(),
            datum_za_obdobi.month() as u8,
            "USD",
        )
        .unwrap();
        (
            kurz_usd,
            amount_usd,
            (amount_usd * kurz_usd).ceil() as i64,
            (amount_usd * kurz_usd * 0.21).ceil() as i64,
        )
    } else {
        let amount_czk = prijeti_sluzeb_v_jinem_state_czk.unwrap();
        let (base_czk, dph_czk) = amount_czk_and_dph(amount_czk);
        (0.0, 0.0, base_czk, dph_czk)
    };

    let (_, dph_prijata_zdanitelna_plneni_nad_limit_czk) =
        amount_czk_and_dph(prijata_zdanitelna_plneni_nad_limit_czk);

    Config {
        datum_podpisu: today,
        datum_za_obdobi,
        kurz_eur,
        kurz_usd,
        hodnota_plneni_eur,
        hodnota_plneni_czk: (hodnota_plneni_eur * kurz_eur).ceil() as i64,
        prijata_zdanitelna_plneni_czk,
        dph_prijata_zdanitelna_plneni_czk: prijata_zdanitelna_plneni_czk * 0.21,
        prijata_zdanitelna_plneni_nad_limit_czk,
        dph_prijata_zdanitelna_plneni_nad_limit_czk: dph_prijata_zdanitelna_plneni_nad_limit_czk
            as f64,
        prijata_zdanitelna_plneni_nad_limit_dic,
        prijata_zdanitelna_plneni_nad_limit_doklad,
        prijata_zdanitelna_plneni_nad_limit_dppd,
        prijeti_sluzeb_v_jinem_state_usd,
        prijeti_sluzeb_v_jinem_state_czk,
        dph_prijeti_sluzeb_v_jinem_state_czk,
    }
}

fn main() {
    let config = parse_args();
    println!("Config v MAINU {:?}!", config);

    generate_sh(&config);
    generate_kh(&config);
    generate_dph(&config);
}

#[cfg(test)]
mod tests {
    use super::{amount_czk_and_dph, dic_without_country_code, parse_czech_date, parse_iso_date};
    use time::{Date, Month};

    #[test]
    fn test_parse_iso_date_valid() {
        let d = parse_iso_date("2026-01-16");
        assert_eq!(
            d,
            Date::from_calendar_date(2026, Month::January, 16).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_iso_date_invalid_format() {
        let _ = parse_iso_date("16.01.2026");
    }

    #[test]
    fn test_amount_czk_and_dph_rounds_openai_invoice() {
        assert_eq!(amount_czk_and_dph(412.40), (413, 87));
    }

    #[test]
    fn test_amount_czk_and_dph_rounds_large_domestic_invoice() {
        assert_eq!(amount_czk_and_dph(19865.29), (19866, 4172));
    }

    #[test]
    fn test_parse_czech_date() {
        let d = parse_czech_date("25.3.2026");
        assert_eq!(d, Date::from_calendar_date(2026, Month::March, 25).unwrap());
    }

    #[test]
    fn test_dic_without_country_code() {
        assert_eq!(dic_without_country_code("CZ27082440"), "27082440");
        assert_eq!(dic_without_country_code("27082440"), "27082440");
    }
}
