use std::collections::HashMap;

use super::id::HasID;
use super::input_value::InputValue;
use super::position::{HasPosition, MutPosition, Position};
use super::room::InRoom;
use iced::widget::image::Handle;
use iced::widget::Image;
use magnus::value::Qnil;
use magnus::QNIL;
use magnus::{
    block, block::Proc, class, define_class, define_module, function, method, prelude::*, Error,
    Symbol,
};

#[magnus::wrap(class = "Character")]
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
    sprite: Image,
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

    pub fn get_image(&self) -> &Image {
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
            Some(action) => action.call(()),
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

/// This is the ruby class definition for the Character class.
pub fn init() -> Result<(), magnus::Error> {
    let class = define_class("Character", class::object())?;
    class.define_method("ref_id", method!(Character::ref_id, 0))?;
    //class.define_method("set_room!", method!(Character::set_room, 1))?;
    Ok(())
}
