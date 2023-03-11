use super::id::HasID;
use super::position::{HasPosition, Position};
use super::room::InRoom;
use magnus::{class, define_class, define_module, function, method, prelude::*, Error};

#[magnus::wrap(class = "RBQuest::Character")]
pub struct Character {
    id: String,
    room: Option<String>,
    pos: Position,
}

impl Character {
    pub fn set_room(&mut self, room_id: &str) {
        self.room = Some(room_id.clone().to_string())
    }

    pub fn no_room(&mut self) {
        self.room = None
    }
}

impl HasID for Character {
    fn ref_id(&self) -> &str {
        &self.id
    }
}

impl HasPosition for Character {
    fn pos(&self) -> &Position {
        &self.pos
    }
}

impl InRoom for Character {
    fn room(&self) -> Option<&str> {
        match &self.room {
            Some(r) => Some(r),
            None => None,
        }
    }
}

pub fn init() -> Result<(), magnus::Error> {
    let class = define_class("Character", class::object())?;
    class.define_method("ref_id", method!(Character::ref_id, 0))?;
    //class.define_method("set_room!", method!(Character::set_room, 1))?;
    Ok(())
}
