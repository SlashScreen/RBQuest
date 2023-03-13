use std::cell::RefCell;

use magnus::{
    class, define_class, function, init, method, module, Attr, Class, Error, Module, Object,
};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[magnus::wrap(class = "Position", free_immediately, size)]
pub struct MutPosition(pub std::cell::RefCell<Position>);

impl MutPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(RefCell::new(Position { x, y, z }))
    }

    pub fn distance(&self, other: &MutPosition) -> f64 {
        (((other.0.borrow().x - self.0.borrow().x).powf(2.0)
            + (other.0.borrow().y - self.0.borrow().y).powf(2.0)) as f64)
            .sqrt()
    }

    pub fn distancesq(&self, other: &MutPosition) -> f64 {
        (other.0.borrow().x - self.0.borrow().x).powf(2.0)
            + (other.0.borrow().y - self.0.borrow().y).powf(2.0)
    }

    pub fn x(&self) -> f64 {
        self.0.borrow().x
    }

    pub fn set_x(&self, val: f64) {
        self.0.borrow_mut().x = val
    }

    pub fn y(&self) -> f64 {
        self.0.borrow().y
    }

    pub fn set_y(&self, val: f64) {
        self.0.borrow_mut().y = val
    }

    pub fn z(&self) -> f64 {
        self.0.borrow().z
    }

    pub fn set_z(&self, val: f64) {
        self.0.borrow_mut().z = val
    }
}

pub trait HasPosition {
    fn pos(&self) -> MutPosition;
}

pub fn init() -> Result<(), Error> {
    let class = define_class("Position", class::object())?;
    class.define_singleton_method("new", function!(MutPosition::new, 3))?;
    class.define_method("distance", method!(MutPosition::distance, 1))?;
    class.define_method("distancesq", method!(MutPosition::distancesq, 1))?;
    class.define_method("x", method!(MutPosition::x, 0))?;
    class.define_method("set_x", method!(MutPosition::set_x, 1))?;
    class.define_method("y", method!(MutPosition::y, 0))?;
    class.define_method("set_y", method!(MutPosition::set_y, 1))?;
    class.define_method("z", method!(MutPosition::z, 0))?;
    class.define_method("set_z", method!(MutPosition::set_z, 1))?;
    Ok(())
}
