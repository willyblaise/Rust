use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub city: String,
    pub occupation: String,
    pub age: i64,
    pub education: String,
}

#[derive(Deserialize, Validate)]
pub struct CreatePerson {
    #[validate(length(min = 2))]
    pub name: String,

    #[validate(length(min = 2))]
    pub city: String,

    #[validate(length(min = 2))]
    pub occupation: String,

    #[validate(range(min = 1, max = 120))]
    pub age: i64,

    #[validate(length(min = 2))]
    pub education: String,
}

