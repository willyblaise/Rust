use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{Row, SqlitePool};
use crate::models::{Keyboard, CreateKeyboard, ApiResponse};
use validator::Validate;


pub async fn list_keyboards(State(pool): State<SqlitePool>) -> Json<ApiResponse<Vec<Keyboard>>> {
    let rows = sqlx::query(
        r#"SELECT id, brand, model, switch_type, key_count, connection FROM keyboards"#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let keyboards = rows.into_iter().map(|row| Keyboard {
        id: row.get("id"),
                                         brand: row.get("brand"),
                                         model: row.get("model"),
                                         switch_type: row.get("switch_type"),
                                             key_count: row.get("key_count"),
                                         connection: row.get("connection"),
    }).collect();

    Json(ApiResponse { data: keyboards })
}

pub async fn get_keyboard_by_id(
    State(pool): State<SqlitePool>,
                                Path(id): Path<i64>,
) -> Json<ApiResponse<Keyboard>> {
    let row = sqlx::query(
        r#"SELECT id, brand, model, switch_type, key_count, connection
        FROM keyboards WHERE id = ?"#
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(ApiResponse {
        data: Keyboard {
            id: row.get("id"),
         brand: row.get("brand"),
         model: row.get("model"),
         switch_type: row.get("switch_type"),
             key_count: row.get("key_count"),
         connection: row.get("connection"),
        }
    })
}

pub async fn create_keyboard(
    State(pool): State<SqlitePool>,
                             Json(payload): Json<CreateKeyboard>,
) -> Json<ApiResponse<Keyboard>> {
    payload.validate().unwrap();

    let result = sqlx::query(
        r#"INSERT INTO keyboards (brand, model, switch_type, key_count, connection)
    VALUES (?1, ?2, ?3, ?4, ?5)"#
    )
    .bind(&payload.brand)
    .bind(&payload.model)
    .bind(&payload.switch_type)
    .bind(payload.key_count)
    .bind(&payload.connection)
    .execute(&pool)
    .await
    .unwrap();

    let id = result.last_insert_rowid();

    Json(ApiResponse {
        data: Keyboard {
            id,
            brand: payload.brand,
            model: payload.model,
            switch_type: payload.switch_type,
                key_count: payload.key_count,
                connection: payload.connection,
        }
    })
}
