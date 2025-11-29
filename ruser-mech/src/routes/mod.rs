pub mod people;
pub mod keyboard;

pub use people::{list_people, create_person, get_person_by_id};
pub use keyboard::{list_keyboards, create_keyboard, get_keyboard_by_id};

