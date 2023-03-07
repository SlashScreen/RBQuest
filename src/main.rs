use magnus::{embed, eval};

mod core;

fn main() {
    let _cleanup = unsafe { magnus::embed::init() };

    eval::<bool>("5.times { |i| puts i }").unwrap();
}
