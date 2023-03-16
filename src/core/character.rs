use std::cell::Ref;
use std::collections::HashMap;

use super::id::HasID;
use super::input_value::InputValue;
use super::position::{HasPosition, MutPosition, Position};
use super::room::InRoom;
use magnus::value::Qnil;
use magnus::QNIL;
use magnus::{
    block, block::Proc, class, define_class, define_module, function, method, prelude::*, Error,
    Symbol,
};

/// A character object. A character is any object that can be rendered to the screen, and can be interacted with and moved around.
/// It's meant for NPCs, but in theory it can be for anything.
pub struct Character {
    /// The character's ID.
    id: String,
    /// The room the character is in. None if the character is not in a room.
    room: Option<String>,
    /// The character's position.
    pos: Position,
    /// The character's sprite.
    sprite: Box<[u8]>,
    /// The character's input callback procs.
    input_callbacks: HashMap<String, Proc>,
}

impl Character {
    pub fn set_room(&mut self, room_id: &str) {
        /*!
         Set the character's room.
        */
        self.room = Some(room_id.clone().to_string())
    }

    pub fn clear_room(&mut self) {
        /*!
         * Remove the character from any room.
         */
        self.room = None
    }

    pub fn get_image(&self) -> &[u8] {
        /*!
         * This is a callback for the renderer. It is called to get the sprite for this character.
         */
        &self.sprite
    }

    pub fn input(&mut self, event: Symbol, proc: Proc) {
        /*! This is a callback for the input system. It is called to register an input event callback.
        This is exposed to ruby as the `self.input` method, and takes a symbol and a block.
        */
        self.input_callbacks.insert(event.to_string(), proc);
    }

    pub fn on_input(&self, event: Symbol, val: InputValue) -> Result<Qnil, Error> {
        /*! This is a callback for the input system. It is called when an input event is triggered.*/
        //TODO: val
        match self.input_callbacks.get(&event.to_string()) {
            Some(action) => action.call(val.into_rarray()),
            None => Ok(QNIL),
        }
    }
}

impl HasID for Character {
    /// Get the character's ID.
    fn ref_id(&self) -> &str {
        &self.id
    }
}

impl HasPosition for Character {
    /// Get the character's position.
    fn pos(&self) -> MutPosition {
        MutPosition(self.pos.into())
    }
}

impl InRoom for Character {
    /// Get the character's room.
    fn room(&self) -> Option<&str> {
        match &self.room {
            Some(r) => Some(r),
            None => None,
        }
    }
}

#[magnus::wrap(class = "Character")]
pub struct MutCharacter(pub std::cell::RefCell<Character>);

impl MutCharacter {
    pub fn new(id: &str, pos: Position, sprite: Box<[u8]>) -> Self {
        MutCharacter(std::cell::RefCell::new(Character {
            id: id.to_string(),
            room: None,
            pos,
            sprite,
            input_callbacks: HashMap::new(),
        }))
    }

    pub fn set_room(&self, room_id: String) {
        self.0.borrow_mut().set_room(&room_id)
    }

    pub fn clear_room(&self) {
        self.0.borrow_mut().clear_room()
    }

    pub fn input(&self, event: Symbol, proc: Proc) {
        self.0.borrow_mut().input(event, proc)
    }

    pub fn on_input(&self, event: Symbol, val: InputValue) {
        self.0.borrow().on_input(event, val).unwrap();
    }

    fn ref_id(&self) -> String {
        Ref::map(self.0.borrow(), |x| x.ref_id()).to_string()
    }

    fn room(&self) -> Option<String> {
        match self.0.borrow().room() {
            Some(r) => Some(r.to_string()),
            None => None,
        }
    }
}

impl HasPosition for MutCharacter {
    fn pos(&self) -> MutPosition {
        MutPosition(self.0.borrow().pos.into())
    }
}

/// This is the ruby class definition for the Character class.
pub fn init() -> Result<(), magnus::Error> {
    let class = define_class("Character", class::object())?;
    class.define_method("ref_id", method!(MutCharacter::ref_id, 0))?;
    class.define_method("pos", method!(MutCharacter::pos, 0))?;
    class.define_method("room", method!(MutCharacter::room, 0))?;
    class.define_method("input", method!(MutCharacter::input, 2))?;
    //class
    //    .singleton_class()?
    //    .define_method("on_input", function!(MutCharacter::on_input, 2))?;
    class.define_method("clear_room!", method!(MutCharacter::clear_room, 0))?;
    class.define_method("room=", method!(MutCharacter::set_room, 1))?;
    Ok(())
}
