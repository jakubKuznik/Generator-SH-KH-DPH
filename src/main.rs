use std::env;
use std::process::exit;
use generator_sh_kh_dph::dph::generate_dph;
use generator_sh_kh_dph::Config;
use generator_sh_kh_dph::rates::get_monthly_average_czk_rate;
use generator_sh_kh_dph::sh::generate_sh;
use generator_sh_kh_dph::kh::generate_kh;
use generator_sh_kh_dph::date_func::{one_month_earlier,get_today};
use time::Date;
use time::format_description;

fn usage() -> &'static str {
    "Usage: app --hodnota-plneni <EUR> --prijata-zdanitelna-plneni <CZK> --prijeti-sluzeb-v-jinem-state <USD> [--datum-za-obdobi <YYYY-MM-DD>]\n
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
        --prijeti-sluzeb-v-jinem-state <USD>\n
            Částka BEZ DPH!\n 
            licence chatGPT,
        "
}

fn parse_iso_date(input: &str) -> Date {
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    Date::parse(input, &format).unwrap_or_else(|e| {
        panic!("Invalid date '{}', expected YYYY-MM-DD: {}", input, e);
    })
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
    let mut prijeti_sluzeb_v_jinem_state_usd: Option<f64> = None;

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
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            "--prijeti-sluzeb-v-jinem-state" => {
                if let Some(val) = args.next() {
                    prijeti_sluzeb_v_jinem_state_usd = Some(val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --prijeti-sluzeb-v-jinem-state '{}': {}", val, e);
                    }));
                } else {
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            _ => println!("Unknown arg: {}", arg),
        }
    }

    let kurz_eur = get_monthly_average_czk_rate(datum_za_obdobi.year(), datum_za_obdobi.month() as u8, "EUR").unwrap();
    let kurz_usd = get_monthly_average_czk_rate(datum_za_obdobi.year(), datum_za_obdobi.month() as u8, "USD").unwrap();

    let hodnota_plneni_eur = hodnota_plneni_eur.unwrap_or_else(|| {
        panic!("Error: --hodnota-plneni requires a positive int value");
    });
    let prijata_zdanitelna_plneni_czk = prijata_zdanitelna_plneni_czk.unwrap_or_else(|| {
        panic!("Error: --prijata-zdanitelna-plneni requires a positive int value");
    });
    let prijeti_sluzeb_v_jinem_state_usd = prijeti_sluzeb_v_jinem_state_usd.unwrap_or_else(|| {
        panic!("Error: --prijeti-sluzeb-v-jinem-state requires a positive int value");
    });

    if hodnota_plneni_eur < 0.0 {
        panic!("Error: --hodnota-plneni requires a positive int value, it has: {}", hodnota_plneni_eur);
    }
    if prijata_zdanitelna_plneni_czk < 0.0 {
        panic!("Error: --prijata-zdanitelna-plneni a positive int value, it has: {}", prijata_zdanitelna_plneni_czk);
    }
    if prijeti_sluzeb_v_jinem_state_usd < 0.0 {
        panic!("Error: --prijeti_sluzeb_v_jinem_state_usd a positive int value, it has: {}", prijeti_sluzeb_v_jinem_state_usd);
    }

    Config {
        datum_podpisu: today,
        datum_za_obdobi,
        kurz_eur,
        kurz_usd,
        hodnota_plneni_eur,
        hodnota_plneni_czk: (hodnota_plneni_eur * kurz_eur).ceil() as i64,
        prijata_zdanitelna_plneni_czk,
        dph_prijata_zdanitelna_plneni_czk: prijata_zdanitelna_plneni_czk * 0.21,
        prijeti_sluzeb_v_jinem_state_usd,
        prijeti_sluzeb_v_jinem_state_czk: (prijeti_sluzeb_v_jinem_state_usd * kurz_usd).ceil() as i64,
        dph_prijeti_sluzeb_v_jinem_state_czk: (prijeti_sluzeb_v_jinem_state_usd * kurz_usd * 0.21).ceil() as i64,
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
    use super::parse_iso_date;
    use time::{Date, Month};

    #[test]
    fn test_parse_iso_date_valid() {
        let d = parse_iso_date("2026-01-16");
        assert_eq!(d, Date::from_calendar_date(2026, Month::January, 16).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_parse_iso_date_invalid_format() {
        let _ = parse_iso_date("16.01.2026");
    }
}
