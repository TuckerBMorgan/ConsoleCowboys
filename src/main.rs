#![allow(dead_code)]
extern crate rand;
extern crate cgmath;


mod system;
mod overworldmap;
mod party_manager;
use system::System;

fn main() {
    let mut system = System::new();
    system.update();
}
