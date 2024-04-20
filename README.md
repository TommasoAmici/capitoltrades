# Capitol trades bot

This repository contains the source code for a Telegram bot to track trades
of American politicians.

The [capitoltrades_api](./crates/capitoltrades_api/) crate is a standalone
client for fetching data from <https://www.capitoltrades.com>.
It uses [reqwest](https://docs.rs/reqwest/latest/reqwest/) for synchronous HTTP requests.

The [telegram_bot](./crates/telegram_bot/) crate contains the code for the Telegram
bot, built with [Teloxide](https://github.com/teloxide/teloxide).
