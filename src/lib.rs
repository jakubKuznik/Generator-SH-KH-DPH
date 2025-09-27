pub mod date_func;
pub mod rates;
pub mod sh;
pub mod kh;
pub mod dph;

use time::Date;

#[derive(Debug)]
pub struct Config {
    pub datum_podpisu: Date,
    pub datum_za_obdobi: Date,
    pub kurz_eur: f64,
    pub kurz_usd: f64,
    pub hodnota_plneni_eur: f64,
    pub hodnota_plneni_czk: i64,
    pub prijata_zdanitelna_plneni_czk: f64,
    pub dph_prijata_zdanitelna_plneni_czk: f64,
    pub prijeti_sluzeb_v_jinem_state_usd: f64,
    pub prijeti_sluzeb_v_jinem_state_czk: i64,
    pub dph_prijeti_sluzeb_v_jinem_state_czk: i64,
}
