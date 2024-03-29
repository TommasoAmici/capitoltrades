use super::actions::Action;
use crate::errors::HandlerResult;
use crate::markdown::trades::trade_to_markdown;
use capitoltrades_api::types::{Response, Trade};
use capitoltrades_api::{Client, Query, SortDirection, TradeQuery, TradeSortBy};
use teloxide::payloads::EditMessageTextSetters;
use teloxide::types::InlineKeyboardButtonKind;
use teloxide::{
    adaptors::Throttle,
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ParseMode},
    Bot,
};

fn list_keyboard(query: &TradeQuery) -> InlineKeyboardMarkup {
    let buttons = vec![
        ("Sort by publication date", TradeSortBy::PublicationDate),
        ("Sort by trade date", TradeSortBy::TradeDate),
        ("Sort by reporting gap", TradeSortBy::ReportingGap),
    ];
    let mut keyboard = InlineKeyboardMarkup::default();
    for (label, sort_by) in buttons {
        keyboard = keyboard.append_row(vec![InlineKeyboardButton::new(
            format!("{} Desc", label),
            InlineKeyboardButtonKind::CallbackData(format!(
                "{}:{}:{}:{}",
                Action::TradesList as u8,
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
                Action::TradesList as u8,
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
            Action::TradesList as u8,
            query.sort_by as u8,
            query.common.sort_direction as u8,
            query.common.page + 1,
        )),
    ));
    keyboard.append_row(pagination_row)
}

fn text_from_response(response: &Response<Trade>) -> String {
    let mut text = String::new();
    for trade in &response.data {
        text.push_str(&trade_to_markdown(&trade));
        text.push_str("\n\n");
    }
    text
}

pub async fn list_callback(bot: Throttle<Bot>, msg: Message, payload: &str) -> HandlerResult {
    let payload: Vec<&str> = payload.split(":").collect();
    let sort_by: TradeSortBy = payload[0].parse().expect("Invalid sort by");
    let sort_direction: SortDirection = payload[1].parse().expect("Invalid sort direction");
    let page: i64 = payload[2].parse().expect("Invalid page");

    let client = Client::new();
    let query = TradeQuery::default()
        .with_sort_by(sort_by)
        .with_sort_direction(sort_direction)
        .with_page(page);
    let response = client.get_trades(&query).await?;
    let text = text_from_response(&response);
    match bot
        .edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(list_keyboard(&query))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to change trades sorting: {}", e);
            Err(e.into())
        }
    }
}

pub async fn list(bot: Throttle<Bot>, msg: Message) -> HandlerResult {
    let client = Client::new();
    let query = TradeQuery::default();
    let response = client.get_trades(&query).await?;
    let text = text_from_response(&response);
    match bot
        .send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(list_keyboard(&query))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to list trades: {}", e);
            Err(e.into())
        }
    }
}
