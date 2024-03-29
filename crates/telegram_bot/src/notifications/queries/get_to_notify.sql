SELECT chat_id
FROM tracked_politicians
WHERE politician_id = ?1
  AND chat_id NOT IN (
    SELECT chat_id
    FROM notified_trades
    WHERE tx_id = ?3
  )
UNION
SELECT chat_id
FROM tracked_issuers
WHERE issuer_id = ?2
  AND chat_id NOT IN (
    SELECT chat_id
    FROM notified_trades
    WHERE tx_id = ?3
  )
