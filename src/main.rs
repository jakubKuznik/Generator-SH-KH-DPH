use std::env;
use std::process::exit;
use Generator_SH_KH_DPH::Config;
use Generator_SH_KH_DPH::rates::get_monthly_average_czk_rate;
use Generator_SH_KH_DPH::SH::generate_SH;
use Generator_SH_KH_DPH::date_func::{one_month_earlier,get_today};

fn usage() -> &'static str {
    "Usage: app --hodnota-plneni <POS_INT>\n
    Options:\n
        -h, --help Show this help\n
        --hodnota-plneni <N> Amount in CZK (positive integer)\n"
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
        hodnota_plneni_czk: -1,
        datum_podpisu: today,
        datum_za_obdobi: month_before,
        kurz_eur: get_monthly_average_czk_rate(month_before.year(), month_before.month() as u8, "EUR").unwrap(),
        kurz_usd: get_monthly_average_czk_rate(month_before.year(), month_before.month() as u8, "USD").unwrap(),
    };

    while let Some(arg) = args.next() {
        match arg.to_lowercase().as_str() {
            "-h" | "--help" => {
                println!("{}", usage());
                exit(0);
            }
            "--hodnota-plneni" => {
                if let Some(val) = args.next() {
                    config.hodnota_plneni_czk = val.parse().unwrap_or_else(|e| {
                        panic!("Invalid --hodnota-plneni '{}': {}", val, e);
                    });
                } else {
                    panic!("Error: --hodnota-plneni requires a positive int value");
                }
            }
            _ => println!("Unknown arg: {}", arg),
        }
    }

    if config.hodnota_plneni_czk <= 0 {
        panic!("Error: --hodnota-plneni requires a positive int value");
    }
    config
}

fn main() {
    let config = parse_args();
    println!("Config v MAINU {:?}!", config);

    generate_SH(&config);
}
