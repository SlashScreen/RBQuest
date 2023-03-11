use magnus::{
    class, define_class, function, init, method, module, Attr, Class, Error, Module, Object,
};

#[magnus::wrap(class = "Position", free_immediately, size)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Position) -> f64 {
        (((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)) as f64).sqrt()
    }

    pub fn distancesq(&self, other: &Position) -> f64 {
        (other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)
    }
}

pub trait HasPosition {
    fn pos(&self) -> &Position;
}

pub fn init() -> Result<(), Error> {
    let class = define_class("Position", class::object())?;
    class.define_singleton_method("new", function!(Position::new, 3))?;
    class.define_method("distance", method!(Position::distance, 1))?;
    class.define_method("distancesq", method!(Position::distancesq, 1))?;
    Ok(())
}
