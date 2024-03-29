use sqlx::{Error, SqlitePool};

pub async fn get_to_notify(
    pool: &SqlitePool,
    politician_id: &str,
    issuer_id: i64,
    tx_id: i64,
) -> Result<Vec<i64>, Error> {
    sqlx::query_file_scalar!(
        "src/notifications/queries/get_to_notify.sql",
        politician_id,
        issuer_id,
        tx_id,
    )
    .fetch_all(pool)
    .await
}
