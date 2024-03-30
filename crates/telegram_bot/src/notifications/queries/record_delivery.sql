INSERT INTO notified_trades (chat_id, tx_id)
VALUES (?1, ?2) ON CONFLICT DO NOTHING;
