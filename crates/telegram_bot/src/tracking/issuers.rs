use sqlx::{Error, SqlitePool};

pub async fn get_tracked_issuers(pool: &SqlitePool, chat_id: i64) -> Result<Vec<i64>, Error> {
    sqlx::query_scalar!(
        r#"SELECT issuer_id AS "issuer_id: i64" FROM tracked_issuers WHERE chat_id = ?"#,
        chat_id
    )
    .fetch_all(pool)
    .await
}

pub async fn track_issuer(pool: &SqlitePool, chat_id: i64, issuer_id: i64) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tracked_issuers (chat_id, issuer_id) VALUES (?, ?)",
        chat_id,
        issuer_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn untrack_issuer(pool: &SqlitePool, chat_id: i64, issuer_id: i64) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM tracked_issuers WHERE chat_id = ? AND issuer_id = ?",
        chat_id,
        issuer_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
