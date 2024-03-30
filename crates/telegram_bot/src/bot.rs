use capitoltrades_telegram_bot::{
    endpoints::{self, actions::Action},
    errors::HandlerResult,
};
use sqlx::SqlitePool;
use teloxide::{
    adaptors::{throttle::Limits, Throttle},
    prelude::*,
    utils::command::BotCommands,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Display help message.")]
    Help,
    #[command(description = "List politicians.")]
    Politicians,
    #[command(
        description = "Search politicians. The command also allows you to start tracking their trades."
    )]
    PoliticiansSearch(String),
    #[command(description = "List issuers.")]
    Issuers,
    #[command(description = "Search issuers. The command also allows you to start tracking them.")]
    IssuersSearch(String),
    #[command(description = "List trades.")]
    Trades,
}

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

    let bot = Bot::from_env().throttle(Limits::default());
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set commands");
    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(commands_handler);
    let callback_query_handler = Update::filter_callback_query().endpoint(callback_query_handler);
    let handler = dptree::entry()
        .branch(command_handler)
        .branch(callback_query_handler);
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn commands_handler(
    bot: Throttle<Bot>,
    msg: Message,
    cmd: Command,
    pool: SqlitePool,
) -> HandlerResult {
    match cmd {
        Command::Help | Command::Start => help_endpoint(bot, msg).await,
        Command::Politicians => endpoints::politicians::list(bot, msg).await,
        Command::PoliticiansSearch(query) => {
            endpoints::politicians::search(bot, msg, &query, &pool).await
        }
        Command::Issuers => endpoints::issuers::list(bot, msg).await,
        Command::IssuersSearch(query) => endpoints::issuers::search(bot, msg, &query, &pool).await,
        Command::Trades => endpoints::trades::list(bot, msg).await,
    }
}

async fn help_endpoint(bot: Throttle<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn callback_query_handler(
    bot: Throttle<Bot>,
    q: CallbackQuery,
    pool: SqlitePool,
) -> HandlerResult {
    let text = q.data;
    if text.is_none() {
        return Ok(());
    }
    let text = text.unwrap();

    let message = q.message;
    if message.is_none() {
        return Ok(());
    }
    let message = message.unwrap();

    let (action, payload) = text.split_once(":").expect("Invalid callback query");
    let parsed_action: Action = action.parse::<Action>().expect("Invalid action");

    match parsed_action {
        Action::PoliticiansList => {
            endpoints::politicians::list_callback(bot, message, payload).await
        }
        Action::PoliticiansSearch => {
            endpoints::politicians::search_callback(bot, message, payload, &pool).await
        }
        Action::IssuersList => endpoints::issuers::list_callback(bot, message, payload).await,
        Action::IssuersSearch => {
            endpoints::issuers::search_callback(bot, message, payload, &pool).await
        }
        Action::TradesList => endpoints::trades::list_callback(bot, message, payload).await,
    }
}
