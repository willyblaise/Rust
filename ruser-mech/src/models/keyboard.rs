use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct Keyboard {
    pub id: i64,
    pub brand: String,
    pub model: String,
    pub switch_type: String,
    pub key_count: i64,
    pub connection: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateKeyboard {
    #[validate(length(min = 2))]
    pub brand: String,

    #[validate(length(min = 1))]
    pub model: String,

    #[validate(length(min = 2))]
    pub switch_type: String,

    #[validate(range(min = 20, max = 120))]
    pub key_count: i64,

    #[validate(length(min = 2))]
    pub connection: String,
}
