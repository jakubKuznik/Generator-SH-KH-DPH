use std::env;
use std::process::exit;

use Generator_SH_KH_DPH::*; // imports your lib.rs

fn usage() -> &'static str {
    "Usage: app --hodnota-plneni <POS_INT>"
}

fn parse_args() -> Config {
    let mut args = env::args();
    if args.len() <= 2 {
        println!("{}", usage());
        exit(0);
    }

    let mut config: Config = Config {
        hodnota_plneni_czk: -1,
        datum_podpisu: get_today(),
        datum_za_obdobi: one_month_earlier(get_today()),
        kurz_eur: 0.0,
        kurz_usd: 0.0,
    };

    while let Some(arg) = args.next() {
        if arg == "--hodnota-plneni" {
            if let Some(val) = args.next() {
                config.hodnota_plneni_czk = val.parse().unwrap();
            }
        }
    }

    config
}

fn main() {
    let config = parse_args();
    println!("Config: {:?}", config);
}
