use anyhow::Result;
use sqlx::SqlitePool;

pub async fn create_people_table(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            city TEXT NOT NULL,
            occupation TEXT NOT NULL,
            age INTEGER NOT NULL,
            education TEXT NOT NULL
        );
    "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_keyboards_table(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS keyboards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            brand TEXT NOT NULL,
            model TEXT NOT NULL,
            switch_type TEXT NOT NULL,
            key_count INTEGER NOT NULL,
            connection TEXT NOT NULL
        );
    "#
    )
    .execute(pool)
    .await?;
    Ok(())
}
