use std::collections::HashMap;
use super::character::Character;

pub struct Game{
    pub characters:HashMap<String, Character>
}

impl Game{
    pub fn add_character(&mut self, id:String, ch:Character){
        self.characters.insert(id, ch);
    }
}
