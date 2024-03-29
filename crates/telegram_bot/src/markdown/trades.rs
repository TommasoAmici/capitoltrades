use crate::markdown::politicians::politician_to_markdown;
use capitoltrades_api::types::{Asset, Trade};
use teloxide::utils::markdown::escape;

fn asset_to_markdown(asset: &Asset) -> String {
    match asset.asset_ticker {
        Some(ref asset_ticker) => format!(
            "*{}* \\({}\\)",
            escape(asset_ticker.as_str()),
            escape(asset.asset_type.as_str()),
        ),
        None => escape(asset.asset_type.as_str()),
    }
}

pub fn trade_to_markdown(trade: &Trade) -> String {
    format!(
        "{} sold {} units of {} {} on {}\\.\nPub date: {}\\. Reporting gap: {} days\\. [Filing link]({})\\.",
        politician_to_markdown(&trade.politician),
        match trade.size {
            Some(size) => size.to_string(),
            None => "an unknown number of".to_string(),
        },
        asset_to_markdown(&trade.asset),
        match trade.price{
            Some(price) => format!("at a price of *${}*", escape(price.to_string().as_str())),
            None => "".to_string(),
        },
        escape(trade.tx_date.to_string().as_str()),
        escape(trade.pub_date.date_naive().to_string().as_str()),
        trade.reporting_gap,
        trade.filing_url,
    )
}
