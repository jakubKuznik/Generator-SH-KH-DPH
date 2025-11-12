use std::env;
use std::process::exit;
use generator_sh_kh_dph::dph::generate_dph;
use generator_sh_kh_dph::Config;
use generator_sh_kh_dph::rates::get_monthly_average_czk_rate;
use generator_sh_kh_dph::sh::generate_sh;
use generator_sh_kh_dph::kh::generate_kh;
use generator_sh_kh_dph::date_func::{one_month_earlier,get_today};

fn usage() -> &'static str {
    "Usage: app --hodnota-plneni <EUR> --prijata-zdanitelna-plneni <CZK> --\n
    Options:\n
        -h, --help Show this help\n
        --hodnota-plneni <EUR>\n
        --prijata-zdanitelna-plneni <CZK>\n
            maximalne 10 000czk na jednotlivé položky\n
            Částka BEZ DPH!\n 
            podnajem kancelare, nakup mobilu\n
        --prijeti-sluzeb_v_jinem_state <USD>\n
            Částka BEZ DPH!\n 
            licence chatGPT,
        "
}

fn parse_args() -> Config {
    let mut args = env::args();
    if args.len() <= 2 {
        println!("{}", usage());
        exit(0);
    }

    let today = get_today();
    let month_before = one_month_earlier(today);

    let mut config = Config {
        datum_podpisu: today,
        datum_za_obdobi: month_before,
        kurz_eur: get_monthly_average_czk_rate(month_before.year(), month_before.month() as u8, "EUR").unwrap(),
        kurz_usd: get_monthly_average_czk_rate(month_before.year(), month_before.month() as u8, "USD").unwrap(),
        hodnota_plneni_eur: -1.0,
        hodnota_plneni_czk: -1,
        prijata_zdanitelna_plneni_czk: -1.0,
        dph_prijata_zdanitelna_plneni_czk: -1.0,
        prijeti_sluzeb_v_jinem_state_usd: -1.0,
        prijeti_sluzeb_v_jinem_state_czk: -1,
        dph_prijeti_sluzeb_v_jinem_state_czk: -1,
    };

    while let Some(arg) = args.next() {
        match arg.to_lowercase().as_str() {
            "-h" | "--help" => {
                println!("{}", usage());
                exit(0);
            }
            "--hodnota-plneni" => {
                if let Some(val) = args.next() {
                    config.hodnota_plneni_eur = val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --hodnota-plneni '{}': {}", val, e);
                    });
                    config.hodnota_plneni_czk = (config.hodnota_plneni_eur * config.kurz_eur).ceil() as i64;
                } else {
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            "--prijata-zdanitelna-plneni" => {
                if let Some(val) = args.next() {
                    config.prijata_zdanitelna_plneni_czk = val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --prijata-zdanitelna-plneni '{}': {}", val, e);
                    });
                    config.dph_prijata_zdanitelna_plneni_czk = config.prijata_zdanitelna_plneni_czk * 0.21;
                } else {
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            "--prijeti-sluzeb-v-jinem-state" => {
                if let Some(val) = args.next() {
                    config.prijeti_sluzeb_v_jinem_state_usd= val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --prijeti-sluzeb-v-jinem-state '{}': {}", val, e);
                    });
                    config.prijeti_sluzeb_v_jinem_state_czk = (config.prijeti_sluzeb_v_jinem_state_usd * config.kurz_usd).ceil() as i64;
                    config.dph_prijeti_sluzeb_v_jinem_state_czk = (config.prijeti_sluzeb_v_jinem_state_usd * config.kurz_usd * 0.21).ceil() as i64
                } else {
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            _ => println!("Unknown arg: {}", arg),
        }
    }

    if config.hodnota_plneni_eur < 0.0 {
        panic!("Error: --hodnota-plneni requires a positive int value");
    }
    if config.prijata_zdanitelna_plneni_czk < 0.0 {
        panic!("Error: --prijata-zdanitelna-plneni a positive int value");
    }
    if config.prijeti_sluzeb_v_jinem_state_usd < 0.0 {
        panic!("Error: --prijeti_sluzeb_v_jinem_state_usd a positive int value");
    }
    config
}

fn main() {
    let config = parse_args();
    println!("Config v MAINU {:?}!", config);

    generate_sh(&config);
    generate_kh(&config);
    generate_dph(&config);
}
