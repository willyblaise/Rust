mod db;
mod models;
mod routes;

use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{fs, path::PathBuf};

use crate::routes::*;

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = prepare_database_path()?;
    println!("Using SQLite at: {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // create tables
    db::create_people_table(&pool).await?;
    db::create_keyboards_table(&pool).await?;

    let app = Router::new()
        // people
        .route("/people", get(list_people).post(create_person))
        .route("/people/:id", get(get_person_by_id))

        // keyboards
        .route("/keyboards", get(list_keyboards).post(create_keyboard))
        .route("/keyboards/:id", get(get_keyboard_by_id))

        .with_state(pool);

    println!("Server running at http://127.0.0.1:3003");
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3003").await?,
        app,
    )
    .await?;

    Ok(())
}

/// This stays in main.rs because it depends on the project directory
fn prepare_database_path() -> Result<String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data_dir = root.join("data");

    fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join("app.db");

    Ok(format!("sqlite://{}", db_path.display()))
}
