use magnus::{embed, eval, require};
use std::fs;

mod core;

fn main() {
    let _cleanup = unsafe { magnus::embed::init() }; //Initialize Ruby

    core::init().unwrap(); //Initialize core
                           //require("src/ruby/test.rb").unwrap();

    eval::<bool>(&fs::read_to_string("src/ruby/test.rb").unwrap()).unwrap();
}
