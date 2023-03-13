use std::cell::RefCell;

use magnus::{
    class, define_class, function, init, method, module, prelude::*, rb_sys::AsRawValue,
    typed_data, wrap, Attr, Class, Error, Module, Object, TypedData,
};

use rb_sys::{rb_define_alloc_func, rb_obj_reveal};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[magnus::wrap(class = "Position", free_immediately, size)]
pub struct MutPosition(pub std::cell::RefCell<Position>);

impl MutPosition {
    // Allocator function to produce a new 'uninitialised' MutPoint.
    //
    // Argument is a Ruby VALUE (pointer to an object) for the class that is
    // being allocated.
    //
    // Return value is a Ruby VALUE (pointer to an object) of the uninitialised
    // object.
    extern "C" fn alloc(class: u64) -> u64 {
        // create the Rust struct. The idea of uninitialised data doesn't
        // really make sense in Rust, so we'll just use default values.
        let point = Self(RefCell::new(Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }));
        // wrap the struct as a Ruby object then get the raw Ruby VALUE pointer
        let instance = typed_data::Obj::wrap(point).as_raw();
        // Magnus' wrap function (which is the only way of wrapping data with
        // and object in Magnus < 0.6) always uses the default class, we need
        // to update that to the passed class, incase it has been subclassed in
        // Ruby.
        unsafe { rb_obj_reveal(instance, class) };
        // Return the instance
        instance
    }

    fn initialize(&self, x: f64, y: f64, z: f64) {
        let mut this = self.0.borrow_mut();
        this.x = x;
        this.y = y;
        this.z = z
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
    // Let Ruby know about our allocator function
    unsafe {
        rb_define_alloc_func(
            // call `class()` from the `TypedData` trait *before* defining
            // our allocator, because in Magnus < 0.6 the default
            // implementation of `TypedData::class()` undefs the allocator
            // function on first call. So we want to get that out of the way
            // before defining the allocator function.
            // It just so happens that `rb_define_alloc_func` wants the same
            // class as an arg, so we can kill two birds with one stone.
            <MutPosition as TypedData>::class().as_raw(),
            Some(MutPosition::alloc),
        )
    };

    class
        .define_method("initialize", method!(MutPosition::initialize, 3))
        .unwrap();
    class.define_method("distance", method!(MutPosition::distance, 1))?;
    class.define_method("distancesq", method!(MutPosition::distancesq, 1))?;
    class.define_method("x", method!(MutPosition::x, 0))?;
    class.define_method("x=", method!(MutPosition::set_x, 1))?;
    class.define_method("y", method!(MutPosition::y, 0))?;
    class.define_method("y=", method!(MutPosition::set_y, 1))?;
    class.define_method("z", method!(MutPosition::z, 0))?;
    class.define_method("z=", method!(MutPosition::set_z, 1))?;
    Ok(())
}
