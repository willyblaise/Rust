use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use std::{fs, path::PathBuf};
use validator::Validate;
use anyhow::Result;

// -----------------------------
// Models
// -----------------------------

#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
}

#[derive(Serialize)]
struct Person {
    id: i64,
    name: String,
    city: String,
    occupation: String,
    age: i64,
    education: String,
}

#[derive(Deserialize, Validate)]
struct CreatePerson {
    #[validate(length(min = 2))]
    name: String,

    #[validate(length(min = 2))]
    city: String,

    #[validate(length(min = 2))]
    occupation: String,

    #[validate(range(min = 1, max = 120))]
    age: i64,

    #[validate(length(min = 2))]
    education: String,
}

// -----------------------------
// Main
// -----------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = prepare_database_path()?;
    println!("Using SQLite at: {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    create_people_table(&pool).await?;

    let app = Router::new()
        .route("/people", get(list_people).post(create_person))
        .route("/people/:id", get(get_person_by_id))
        .with_state(pool);

    println!("Server running at http://127.0.0.1:3002");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3002").await?,
        app,
    )
    .await?;

    Ok(())
}

// -----------------------------
// Handlers
// -----------------------------

async fn list_people(State(pool): State<SqlitePool>) -> Json<ApiResponse<Vec<Person>>> {
    let rows = sqlx::query(
        r#"SELECT id, name, city, occupation, age, education FROM people"#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let people = rows
        .into_iter()
        .map(|row| Person {
            id: row.get("id"),
            name: row.get("name"),
            city: row.get("city"),
            occupation: row.get("occupation"),
            age: row.get("age"),
            education: row.get("education"),
        })
        .collect();

    Json(ApiResponse { data: people })
}

async fn get_person_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Person>> {
    let row = sqlx::query(
        r#"SELECT id, name, city, occupation, age, education FROM people WHERE id = ?"#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let person = Person {
        id: row.get("id"),
        name: row.get("name"),
        city: row.get("city"),
        occupation: row.get("occupation"),
        age: row.get("age"),
        education: row.get("education"),
    };

    Json(ApiResponse { data: person })
}

async fn create_person(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreatePerson>,
) -> Json<ApiResponse<Person>> {
    payload.validate().unwrap();

    let result = sqlx::query(
        r#"INSERT INTO people (name, city, occupation, age, education)
           VALUES (?1, ?2, ?3, ?4, ?5)"#,
    )
    .bind(&payload.name)
    .bind(&payload.city)
    .bind(&payload.occupation)
    .bind(payload.age)
    .bind(&payload.education)
    .execute(&pool)
    .await
    .unwrap();

    let id = result.last_insert_rowid();

    Json(ApiResponse {
        data: Person {
            id,
            name: payload.name,
            city: payload.city,
            occupation: payload.occupation,
            age: payload.age,
            education: payload.education,
        },
    })
}

// -----------------------------
// DB Setup
// -----------------------------

async fn create_people_table(pool: &SqlitePool) -> Result<()> {
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

/// Build a writable SQLite path inside the project directory
fn prepare_database_path() -> Result<String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data_dir = root.join("data");

    // Create ./data folder
    fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("app.db");

    Ok(format!("sqlite://{}", db_path.display()))
}

