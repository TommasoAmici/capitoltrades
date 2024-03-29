use super::actions::Action;
use crate::errors::HandlerResult;
use crate::markdown::politicians::politician_detail_to_markdown;
use crate::tracking::politicians::{get_tracked_politicians, track_politician, untrack_politician};
use crate::tracking::status::Status;
use capitoltrades_api::types::{PoliticianDetail, Response};
use capitoltrades_api::{Client, PoliticianQuery, PoliticianSortBy, Query, SortDirection};
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

fn list_keyboard(query: &PoliticianQuery) -> InlineKeyboardMarkup {
    let buttons = vec![
        (
            "Sort by traded volume",
            PoliticianSortBy::TradedVolume,
            SortDirection::Desc,
        ),
        (
            "Sort by traded issuers count",
            PoliticianSortBy::TradedIssuersCount,
            SortDirection::Desc,
        ),
        (
            "Sort by total trades",
            PoliticianSortBy::TotalTrades,
            SortDirection::Desc,
        ),
        (
            "Sort by date of last trade",
            PoliticianSortBy::DateLastTraded,
            SortDirection::Desc,
        ),
        (
            "Sort by last name",
            PoliticianSortBy::LastName,
            SortDirection::Asc,
        ),
    ];
    let mut keyboard = InlineKeyboardMarkup::default();
    for (label, sort_by, sort_direction) in buttons {
        keyboard = keyboard.append_row(vec![InlineKeyboardButton::new(
            format!("{} Desc", label),
            InlineKeyboardButtonKind::CallbackData(format!(
                "{}:{}:{}:{}",
                Action::PoliticiansList as u8,
                sort_by as u8,
                sort_direction as u8,
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
                Action::PoliticiansList as u8,
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
            Action::PoliticiansList as u8,
            query.sort_by as u8,
            query.common.sort_direction as u8,
            query.common.page + 1,
        )),
    ));
    keyboard.append_row(pagination_row)
}

fn text_from_response(response: &Response<PoliticianDetail>) -> String {
    let mut text = String::new();
    for politician in &response.data {
        text.push_str(&politician_detail_to_markdown(&politician));
        text.push_str("\n\n");
    }
    text
}

pub async fn list_callback(bot: Throttle<Bot>, msg: Message, payload: &str) -> HandlerResult {
    let payload: Vec<&str> = payload.split(":").collect();
    let sort_by: PoliticianSortBy = payload[0].parse().expect("Invalid sort by");
    let sort_direction: SortDirection = payload[1].parse().expect("Invalid sort direction");
    let page: i64 = payload[2].parse().expect("Invalid page");

    let client = Client::new();
    let query = PoliticianQuery::default()
        .with_sort_by(sort_by)
        .with_sort_direction(sort_direction)
        .with_page(page);
    let politicians = client.get_politicians(&query).await?;
    let text = text_from_response(&politicians);
    match bot
        .edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(list_keyboard(&query))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to change politicians sorting: {}", e);
            Err(e.into())
        }
    }
}

pub async fn list(bot: Throttle<Bot>, msg: Message) -> HandlerResult {
    let client = Client::new();
    let query = PoliticianQuery::default();
    let politicians = client.get_politicians(&query).await?;
    let text = text_from_response(&politicians);
    match bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(list_keyboard(&query))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to list politicians: {}", e);
            Err(e.into())
        }
    }
}

fn track_keyboard(politician_id: &str, status: Status) -> InlineKeyboardMarkup {
    let label = match status {
        Status::Tracked => "Untrack",
        Status::Untracked => "Track",
    };
    InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::new(
        label,
        InlineKeyboardButtonKind::CallbackData(format!(
            "{}:{}:{}",
            Action::PoliticiansSearch as u8,
            status as u8,
            politician_id
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
    let politician_id = payload[1];
    match status {
        Status::Tracked => {
            untrack_politician(pool, chat_id, politician_id).await?;
        }
        Status::Untracked => {
            track_politician(pool, chat_id, politician_id).await?;
        }
    };
    bot.edit_message_reply_markup(msg.chat.id, msg.id)
        .reply_markup(track_keyboard(politician_id, status.opposite()))
        .await?;

    Ok(())
}

pub async fn search(
    bot: Throttle<Bot>,
    msg: Message,
    search_query: &str,
    pool: &SqlitePool,
) -> HandlerResult {
    let tracked = get_tracked_politicians(pool, msg.chat.id.0).await?;
    let tracked: HashSet<String> = HashSet::from_iter(tracked.into_iter());
    let client = Client::new();
    let query = PoliticianQuery::default().with_search(search_query);
    let response = client.get_politicians(&query).await?;
    for politician in &response.data {
        let is_tracked = tracked.contains(&politician.politician_id);
        let status = Status::from(is_tracked);
        let text = politician_detail_to_markdown(&politician);
        match bot
            .send_message(msg.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(track_keyboard(&politician.politician_id, status))
            .await
        {
            Ok(_) => continue,
            Err(e) => {
                tracing::error!("Failed to list politicians: {}", e);
            }
        }
    }
    Ok(())
}
