use magnus::Error;
mod character;
mod entity;
mod id;
mod position;
mod room;

pub fn init() -> Result<(), Error> {
    position::init()?;
    character::init()?;
    Ok(())
}
