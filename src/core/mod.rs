use magnus::Error;
mod character;
mod entity;
mod id;
mod input_value;
mod position;
mod room;
mod game;

pub fn init() -> Result<(), Error> {
    position::init()?;
    character::init()?;
    Ok(())
}
