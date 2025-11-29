pub mod people;
pub mod keyboard;

pub use people::*;
pub use keyboard::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}
