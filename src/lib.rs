pub mod date_func;
pub mod rates;
pub mod sh;

use time::Date;

#[derive(Debug)]
pub struct Config {
    pub datum_podpisu: Date,
    pub datum_za_obdobi: Date,
    pub kurz_eur: f64,
    pub kurz_usd: f64,
    pub hodnota_plneni_eur: f64,
    pub hodnota_plneni_czk_rounded: i64,
}
