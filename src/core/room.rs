use super::id::HasID;
use magnus::{class, define_class, define_module, function, method, prelude::*, Error};

pub struct Room {
    id: String,
}

impl HasID for Room {
    fn ref_id(&self) -> &str {
        &self.id
    }
}

pub trait InRoom {
    fn room(&self) -> Option<&str>;
}

pub fn init() -> Result<(), magnus::Error> {
    let class = define_class("Room", class::object())?;
    //class.define_method("ref_id", method!(Room::ref_id, 0))?;
    Ok(())
}
