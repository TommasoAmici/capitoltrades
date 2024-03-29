use super::format_volume;
use capitoltrades_api::types::{Chamber, Party, Politician, PoliticianDetail};
use teloxide::utils::markdown::escape;

pub fn politician_to_markdown(politician: &Politician) -> String {
    escape(
        format!(
            "{} {} ({}, {}, {})",
            &politician.first_name,
            &politician.last_name,
            match politician.party {
                Party::Democrat => "D",
                Party::Republican => "R",
                Party::Other => "O",
            },
            match politician.chamber {
                Chamber::House => "House",
                Chamber::Senate => "Senate",
            },
            politician.state_id.to_uppercase()
        )
        .as_str(),
    )
}

fn politician_stats_to_markdown(politician: &PoliticianDetail) -> String {
    escape(
        format!(
            "Trades: {}. Issuers: {}. Volume: {}. {}",
            politician.stats.count_trades,
            politician.stats.count_issuers,
            format_volume(politician.stats.volume),
            if let Some(d) = politician.stats.date_last_traded {
                format!("Last traded: {}.", d.to_string())
            } else {
                "No trades.".to_string()
            },
        )
        .as_str(),
    )
}

pub fn politician_detail_to_markdown(politician: &PoliticianDetail) -> String {
    format!(
        "{} {} \\({}, {}, {}\\)\n{}",
        escape(&politician.first_name),
        escape(&politician.last_name),
        match politician.party {
            Party::Democrat => "D",
            Party::Republican => "R",
            Party::Other => "O",
        },
        match politician.chamber {
            Chamber::House => "House",
            Chamber::Senate => "Senate",
        },
        politician.state_id.to_uppercase(),
        politician_stats_to_markdown(&politician)
    )
}
