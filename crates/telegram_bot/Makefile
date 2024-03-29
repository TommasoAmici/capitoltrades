include .env
ENV=TELOXIDE_TOKEN=${TELOXIDE_TOKEN} TELEGRAM_CHAT_ID=${TELEGRAM_CHAT_ID} DATABASE_URL=${DATABASE_URL}
run_bot:
	${ENV} cargo run --bin bot
run_notify:
	${ENV} cargo run --bin notify

test:
	cargo test

# DATABASE
.PHONY: db_clean db_migrate db_create db_prepare
DB_NAME = capitoltrades_db

db: db_create db_migrate

db_create: db_clean
	sqlx database create

db_migrate:
	sqlx migrate run

db_create_migration:
	sqlx migrate add $(name)

db_clean:
	rm -f ${DB_NAME}*

db_prepare:
	cargo sqlx prepare --workspace -- --all-features --all-targets
