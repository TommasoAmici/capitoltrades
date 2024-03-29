use sqlx::{Error, SqlitePool};

pub async fn get_tracked_politicians(
    pool: &SqlitePool,
    chat_id: i64,
) -> Result<Vec<String>, Error> {
    sqlx::query_scalar!(
        "SELECT politician_id FROM tracked_politicians WHERE chat_id = ?",
        chat_id
    )
    .fetch_all(pool)
    .await
}

pub async fn track_politician(
    pool: &SqlitePool,
    chat_id: i64,
    politician_id: &str,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tracked_politicians (chat_id, politician_id) VALUES (?, ?)",
        chat_id,
        politician_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn untrack_politician(
    pool: &SqlitePool,
    chat_id: i64,
    politician_id: &str,
) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM tracked_politicians WHERE chat_id = ? AND politician_id = ?",
        chat_id,
        politician_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
