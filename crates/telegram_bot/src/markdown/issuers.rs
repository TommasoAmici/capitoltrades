use super::format_volume;
use capitoltrades_api::types::{IssuerDetail, Performance};
use teloxide::utils::markdown::escape;

fn performance_to_markdown(performance: &Option<Performance>) -> String {
    match performance {
        Some(performance) => {
            format!(
                "\nPrice: {}. 30d: {:.2}%. 90d: {:.2}%. 365d: {:.2}%.",
                match performance.last_price() {
                    Some(price) => price.to_string(),
                    None => "No price data.".to_string(),
                },
                performance.trailing30_change * 100.0,
                performance.trailing90_change * 100.0,
                performance.trailing365_change * 100.0
            )
        }
        None => " No performance data.".to_string(),
    }
}

pub fn issuer_detail_to_markdown(issuer: &IssuerDetail) -> String {
    format!(
        "*{} {}*\\. Last traded: {}\\.\nVolume: {}\\. Trades: {}\\. Politicians: {}\\.{}",
        escape(&issuer.issuer_name),
        escape(
            match &issuer.issuer_ticker {
                Some(ticker) => format!("({}) ", ticker),
                None => "".to_string(),
            }
            .as_str()
        ),
        escape(&issuer.stats.date_last_traded.to_string()),
        escape(format_volume(issuer.stats.volume).as_str()),
        &issuer.stats.count_trades,
        &issuer.stats.count_politicians,
        escape(performance_to_markdown(&issuer.performance).as_str())
    )
}
