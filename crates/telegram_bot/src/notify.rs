use capitoltrades_api::{Client, Query, TradeQuery};
use capitoltrades_telegram_bot::{
    markdown::trades::trade_to_markdown,
    notifications::notified_trades::{get_to_notify, record_delivery},
};
use rand::prelude::*;
use sqlx::SqlitePool;
use std::time::Duration;
use teloxide::{
    adaptors::throttle::Limits,
    payloads::SendMessageSetters,
    requests::{Requester, RequesterExt},
    types::ChatId,
    Bot,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    let mut rng = thread_rng();
    let secs = rng.gen_range(60..120);
    tokio::time::sleep(Duration::from_secs(secs)).await;

    let bot = Bot::from_env().throttle(Limits::default());

    tracing::info!("Getting recent trades");
    let client = Client::new();
    let query = TradeQuery::default()
        .with_page_size(20)
        .with_pub_date_relative(30);
    let recent_trades = client.get_trades(&query).await?;

    for trade in recent_trades.data {
        tracing::info!("Getting chat ids to notify for trade {}", trade.tx_id);
        let should_notify =
            get_to_notify(&pool, &trade.politician_id, trade.issuer_id, trade.tx_id).await?;
        let text = trade_to_markdown(&trade);
        for chat_id in should_notify {
            tracing::info!("Notifying {}", chat_id);
            let sent = bot
                .send_message(ChatId(chat_id), &text)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await;
            match sent {
                Ok(_) => {
                    tracing::info!("Notified {}", chat_id);
                    record_delivery(&pool, chat_id, trade.tx_id).await?;
                }
                Err(e) => {
                    tracing::error!("Failed to notify {}: {}", chat_id, e);
                }
            };
        }
    }

    Ok(())
}
