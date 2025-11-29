use axum::{extract::{Path, State}, Json};
use sqlx::{Row, SqlitePool};
use crate::models::{Person, CreatePerson, ApiResponse};
use validator::Validate;


pub async fn list_people(State(pool): State<SqlitePool>) -> Json<ApiResponse<Vec<Person>>> {
    let rows = sqlx::query(
        r#"SELECT id, name, city, occupation, age, education FROM people"#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let people = rows.into_iter().map(|row| Person {
        id: row.get("id"),
                                      name: row.get("name"),
                                      city: row.get("city"),
                                      occupation: row.get("occupation"),
                                      age: row.get("age"),
                                      education: row.get("education"),
    }).collect();

    Json(ApiResponse { data: people })
}

pub async fn get_person_by_id(
    State(pool): State<SqlitePool>,
                              Path(id): Path<i64>,
) -> Json<ApiResponse<Person>> {
    let row = sqlx::query(
        r#"SELECT id, name, city, occupation, age, education FROM people WHERE id = ?"#
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(ApiResponse {
        data: Person {
            id: row.get("id"),
         name: row.get("name"),
         city: row.get("city"),
         occupation: row.get("occupation"),
         age: row.get("age"),
         education: row.get("education"),
        }
    })
}

pub async fn create_person(
    State(pool): State<SqlitePool>,
                           Json(payload): Json<CreatePerson>,
) -> Json<ApiResponse<Person>> {
    payload.validate().unwrap();

    let result = sqlx::query(
        r#"INSERT INTO people (name, city, occupation, age, education)
    VALUES (?1, ?2, ?3, ?4, ?5)"#
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
        }
    })
}
