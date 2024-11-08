use anyhow::{Context, Result};
use chrono::{prelude::*, Duration};
use sqlx::{Pool, Sqlite};

use model::FeedContent;

pub type StoragePool = Pool<Sqlite>;

pub mod model {
    use serde::Serialize;
    use sqlx::prelude::FromRow;

    #[derive(Clone, FromRow, Serialize)]
    pub struct FeedContent {
        pub feed_id: String,
        pub uri: String,
        pub indexed_at: i64,
        pub cid: String,
    }
}

pub async fn feed_content_insert(
    pool: &StoragePool,
    feed_content: &model::FeedContent,
) -> Result<()> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let now = Utc::now();
    sqlx::query("INSERT OR REPLACE INTO feed_content (feed_id, uri, indexed_at, cid, updated_at) VALUES (?, ?, ?, ?, ?)")
        .bind(&feed_content.feed_id)
        .bind(&feed_content.uri)
        .bind(feed_content.indexed_at)
        .bind(&feed_content.cid)
        .bind(now)
        .execute(tx.as_mut())
        .await.context("failed to insert feed content record")?;

    tx.commit().await.context("failed to commit transaction")
}

pub async fn feed_content_paginate(
    pool: &StoragePool,
    feed_uri: &str,
    limit: Option<u16>,
    cursor: Option<(i64, String)>,
) -> Result<Vec<FeedContent>> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let limit = limit.unwrap_or(20).clamp(1, 100);

    let results = if let Some((indexed_at, cid)) = cursor {
        let query = "SELECT * FROM feed_content WHERE feed_id = ? AND (indexed_at, cid) < (?, ?) ORDER BY indexed_at DESC, cid DESC LIMIT ?";

        sqlx::query_as::<_, FeedContent>(query)
            .bind(feed_uri)
            .bind(indexed_at)
            .bind(cid)
            .bind(limit)
            .fetch_all(tx.as_mut())
            .await?
    } else {
        let query = "SELECT * FROM feed_content WHERE feed_id = ? ORDER BY indexed_at DESC, cid DESC LIMIT ?";

        sqlx::query_as::<_, FeedContent>(query)
            .bind(feed_uri)
            .bind(limit)
            .fetch_all(tx.as_mut())
            .await?
    };

    tx.commit().await.context("failed to commit transaction")?;

    Ok(results)
}

pub async fn consumer_control_insert(pool: &StoragePool, source: &str, time_us: i64) -> Result<()> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let now = Utc::now();
    sqlx::query(
        "INSERT OR REPLACE INTO consumer_control (source, time_us, updated_at) VALUES (?, ?, ?)",
    )
    .bind(source)
    .bind(time_us)
    .bind(now)
    .execute(tx.as_mut())
    .await?;

    tx.commit().await.context("failed to commit transaction")
}

pub async fn consumer_control_get(pool: &StoragePool, source: &str) -> Result<Option<i64>> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let result =
        sqlx::query_scalar::<_, i64>("SELECT time_us FROM consumer_control WHERE source = ?")
            .bind(source)
            .fetch_optional(tx.as_mut())
            .await
            .context("failed to select consumer control record")?;

    tx.commit().await.context("failed to commit transaction")?;

    Ok(result)
}

pub async fn verifcation_method_insert(
    pool: &StoragePool,
    did: &str,
    multikey: &str,
) -> Result<()> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let now = Utc::now();
    sqlx::query(
        "INSERT OR REPLACE INTO verification_method_cache (did, multikey, updated_at) VALUES (?, ?, ?)",
    )
    .bind(did)
    .bind(multikey)
    .bind(now)
    .execute(tx.as_mut())
        .await.context("failed to update verification method cache")?;

    tx.commit().await.context("failed to commit transaction")
}

pub async fn verification_method_cleanup(pool: &StoragePool) -> Result<()> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let now = Utc::now();
    let seven_days_ago = now - Duration::days(7);
    sqlx::query("DELETE FROM verification_method_cache WHERE updated_at < ?")
        .bind(seven_days_ago)
        .execute(tx.as_mut())
        .await
        .context("failed to delete old verification method cache records")?;

    tx.commit().await.context("failed to commit transaction")
}

pub async fn verification_method_get(pool: &StoragePool, did: &str) -> Result<Option<String>> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let result = sqlx::query_scalar::<_, String>(
        "SELECT multikey FROM verification_method_cache WHERE did = ?",
    )
    .bind(did)
    .fetch_optional(tx.as_mut())
    .await
    .context("failed to select verification method cache record")?;
    tx.commit().await.context("failed to commit transaction")?;
    Ok(result)
}

pub async fn feed_content_truncate(pool: &StoragePool, feed_id: &str) -> Result<()> {
    let mut tx = pool.begin().await.context("failed to begin transaction")?;

    let result = sqlx::query_scalar::<_, DateTime<Utc>>("SELECT updated_at FROM feed_content WHERE feed_id = ? ORDER BY indexed_at DESC, indexed_at_more DESC LIMIT 1 OFFSET 501")
        .bind(feed_id)
        .fetch_optional(tx.as_mut())
        .await.context("failed select feed content mark record")?;

    if let Some(updated_at) = result {
        sqlx::query("DELETE FROM feed_content WHERE feed_id = ? AND updated_at < ?")
            .bind(feed_id)
            .bind(updated_at)
            .execute(tx.as_mut())
            .await
            .context("failed to delete feed content beyond mark")?;
    }

    tx.commit().await.context("failed to commit transaction")
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn record_feed_content(pool: SqlitePool) -> sqlx::Result<()> {
        let record = super::model::FeedContent {
            feed_id: "feed".to_string(),
            uri: "at://did:plc:qadlgs4xioohnhi2jg54mqds/app.bsky.feed.post/3la3bqjg4hx2n"
                .to_string(),
            indexed_at: 1730673934229172_i64,
            cid: "bafyreih74qdc6zskq7yarqi3xm634vnubf4g3ac5ieegbvakprxpjnsj74".to_string(),
        };
        super::feed_content_insert(&pool, &record)
            .await
            .expect("failed to insert record");

        let records = super::feed_content_paginate(&pool, "feed", None, None)
            .await
            .expect("failed to paginate records");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].feed_id, "feed");
        assert_eq!(
            records[0].uri,
            "at://did:plc:qadlgs4xioohnhi2jg54mqds/app.bsky.feed.post/3la3bqjg4hx2n"
        );
        assert_eq!(records[0].indexed_at, 1730673934229172_i64);

        Ok(())
    }

    #[sqlx::test]
    async fn consumer_control(pool: SqlitePool) -> sqlx::Result<()> {
        super::consumer_control_insert(&pool, "foo", 1730673934229172_i64)
            .await
            .expect("failed to insert record");

        assert_eq!(
            super::consumer_control_get(&pool, "foo")
                .await
                .expect("failed to get record"),
            Some(1730673934229172_i64)
        );

        super::consumer_control_insert(&pool, "foo", 1730673934229173_i64)
            .await
            .expect("failed to insert record");

        assert_eq!(
            super::consumer_control_get(&pool, "foo")
                .await
                .expect("failed to get record"),
            Some(1730673934229173_i64)
        );

        Ok(())
    }
}
