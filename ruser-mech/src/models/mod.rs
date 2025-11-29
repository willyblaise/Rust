pub mod people;
pub mod keyboard;

// Re-export all structs for convenience
pub use people::*;
pub use keyboard::*;

/// Generic API response wrapper
#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}
