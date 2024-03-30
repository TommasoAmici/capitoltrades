use super::actions::Action;
use crate::errors::HandlerResult;
use crate::markdown::issuers::issuer_detail_to_markdown;
use crate::tracking::issuers::{get_tracked_issuers, track_issuer, untrack_issuer};
use crate::tracking::status::Status;
use capitoltrades_api::types::{IssuerDetail, PaginatedResponse};
use capitoltrades_api::{Client, IssuerQuery, IssuerSortBy, Query, SortDirection};
use sqlx::SqlitePool;
use std::collections::HashSet;
use teloxide::payloads::{EditMessageReplyMarkupSetters, EditMessageTextSetters};
use teloxide::types::InlineKeyboardButtonKind;
use teloxide::{
    adaptors::Throttle,
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

fn list_keyboard(query: &IssuerQuery) -> InlineKeyboardMarkup {
    let buttons = vec![
        ("Sort by traded volume", IssuerSortBy::TradedVolume),
        ("Sort by politicians count", IssuerSortBy::PoliticiansCount),
        ("Sort by total trades", IssuerSortBy::TotalTrades),
        ("Sort by date of last trade", IssuerSortBy::DateLastTraded),
        ("Sort by markt cap", IssuerSortBy::MarketCap),
    ];
    let mut keyboard = InlineKeyboardMarkup::default();
    for (label, sort_by) in buttons {
        keyboard = keyboard.append_row(vec![InlineKeyboardButton::new(
            format!("{} Desc", label),
            InlineKeyboardButtonKind::CallbackData(format!(
                "{}:{}:{}:{}",
                Action::IssuersList as u8,
                sort_by as u8,
                SortDirection::Desc as u8,
                query.common.page,
            )),
        )]);
    }
    let mut pagination_row = vec![];
    if query.common.page > 1 {
        pagination_row.push(InlineKeyboardButton::new(
            "<",
            InlineKeyboardButtonKind::CallbackData(format!(
                "{}:{}:{}:{}",
                Action::IssuersList as u8,
                query.sort_by as u8,
                query.common.sort_direction as u8,
                std::cmp::max(1, query.common.page - 1),
            )),
        ))
    }
    pagination_row.push(InlineKeyboardButton::new(
        ">",
        InlineKeyboardButtonKind::CallbackData(format!(
            "{}:{}:{}:{}",
            Action::IssuersList as u8,
            query.sort_by as u8,
            query.common.sort_direction as u8,
            query.common.page + 1,
        )),
    ));
    keyboard.append_row(pagination_row)
}

fn text_from_response(response: &PaginatedResponse<IssuerDetail>) -> String {
    let mut text = String::new();
    for issuer in &response.data {
        text.push_str(&issuer_detail_to_markdown(&issuer));
        text.push_str("\n\n");
    }
    text
}

pub async fn list_callback(bot: Throttle<Bot>, msg: Message, payload: &str) -> HandlerResult {
    let payload: Vec<&str> = payload.split(":").collect();
    let sort_by: IssuerSortBy = payload[0].parse().expect("Invalid sort by");
    let sort_direction: SortDirection = payload[1].parse().expect("Invalid sort direction");
    let page: i64 = payload[2].parse().expect("Invalid page");

    let client = Client::new();
    let query = IssuerQuery::default()
        .with_sort_by(sort_by)
        .with_sort_direction(sort_direction)
        .with_page(page);
    let issuers = client.get_issuers(&query).await?;
    let text = text_from_response(&issuers);
    match bot
        .edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(list_keyboard(&query))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to change issuers sorting: {}", e);
            Err(e.into())
        }
    }
}

pub async fn list(bot: Throttle<Bot>, msg: Message) -> HandlerResult {
    let client = Client::new();
    let query = IssuerQuery::default();
    let issuers = client.get_issuers(&query).await?;
    let text = text_from_response(&issuers);
    match bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(list_keyboard(&query))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to list issuers: {}", e);
            Err(e.into())
        }
    }
}

fn track_keyboard(issuer_id: i64, status: Status) -> InlineKeyboardMarkup {
    let label = match status {
        Status::Tracked => "Untrack",
        Status::Untracked => "Track",
    };
    InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::new(
        label,
        InlineKeyboardButtonKind::CallbackData(format!(
            "{}:{}:{}",
            Action::IssuersSearch as u8,
            status as u8,
            issuer_id
        )),
    )])
}

pub async fn search_callback(
    bot: Throttle<Bot>,
    msg: Message,
    payload: &str,
    pool: &SqlitePool,
) -> HandlerResult {
    let chat_id = msg.chat.id.0;
    let payload: Vec<&str> = payload.split(":").collect();
    let status: Status = payload[0].parse().expect("Invalid value for status");
    let issuer_id: i64 = payload[1].parse().expect("Invalid issuer ID");
    match status {
        Status::Tracked => {
            untrack_issuer(pool, chat_id, issuer_id).await?;
        }
        Status::Untracked => {
            track_issuer(pool, chat_id, issuer_id).await?;
        }
    };
    bot.edit_message_reply_markup(msg.chat.id, msg.id)
        .reply_markup(track_keyboard(issuer_id, status.opposite()))
        .await?;

    Ok(())
}

pub async fn search(
    bot: Throttle<Bot>,
    msg: Message,
    search_query: &str,
    pool: &SqlitePool,
) -> HandlerResult {
    let tracked = get_tracked_issuers(pool, msg.chat.id.0).await?;
    let tracked: HashSet<i64> = HashSet::from_iter(tracked.into_iter());
    let client = Client::new();
    let query = IssuerQuery::default().with_search(search_query);
    let response = client.get_issuers(&query).await?;
    for issuer in &response.data {
        let is_tracked = tracked.contains(&issuer.issuer_id);
        let status = Status::from(is_tracked);
        let text = issuer_detail_to_markdown(&issuer);
        match bot
            .send_message(msg.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(track_keyboard(issuer.issuer_id, status))
            .await
        {
            Ok(_) => continue,
            Err(e) => {
                tracing::error!("Failed to list issuers: {}", e);
            }
        }
    }
    Ok(())
}
