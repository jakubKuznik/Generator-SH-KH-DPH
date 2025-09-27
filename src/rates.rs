// KURZY CNB 
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AveragesResponse {
    averages: Vec<AverageEntry>,
}

#[derive(Debug, Deserialize)]
struct AverageEntry {
    month: String,
    average: f64,
    year: i32,
}

fn month_to_abbr(month: u8) -> &'static str {
    match month {
        1 => "JAN", 2 => "FEB", 3 => "MAR", 4 => "APR", 5 => "MAY", 6 => "JUN",
        7 => "JUL", 8 => "AUG", 9 => "SEP", 10 => "OCT", 11 => "NOV", 12 => "DEC",
        _ => panic!("Invalid month: {}", month),
    }
}

pub fn get_monthly_average_czk_rate(year: i32, month: u8, currency: &str) -> Result<f64, String> {
    let url = format!(
        "https://api.cnb.cz/cnbapi/exrates/monthly-averages-currency?currency={}",
        currency
    );

    let resp: AveragesResponse = Client::new()
        .get(&url)
        .send().map_err(|e| format!("HTTP error: {e}"))?
        .error_for_status().map_err(|e| format!("Status error: {e}"))?
        .json().map_err(|e| format!("JSON parse error: {e}"))?;

    let abbr = month_to_abbr(month);

    resp.averages
        .into_iter()
        .find(|entry| entry.year == year && entry.month == abbr)
        .map(|entry| entry.average)
        .ok_or_else(|| format!("No data for {currency} {year}-{month:02}"))
}
