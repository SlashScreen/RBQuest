use super::position::Position;
use magnus::{class, define_class, function, method, prelude::*, Error};

#[magnus::wrap(class = "Entity")]
pub struct Entity {
    pub pos: Position,
}

impl Entity {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            pos: Position { x, y, z },
        }
    }
}

/* #[magnus::init]
fn init() -> Result<(), Error> {
    let class = define_class("Entity", class::object())?;
    class.define_singleton_method("new", function!(Entity::new, 3))?;
    Ok(())
}
 */
