CREATE TABLE tracked_politicians (
  chat_id INTEGER NOT NULL,
  politician_id TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX tracked_politicians_idx ON tracked_politicians(chat_id, politician_id);
CREATE TABLE tracked_issuers (
  chat_id INTEGER NOT NULL,
  issuer_id INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX tracked_issuers_idx ON tracked_issuers(chat_id, issuer_id);
CREATE TABLE notified_trades (
  chat_id INTEGER NOT NULL,
  tx_id INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX notified_trades_idx ON notified_trades(chat_id, tx_id);
