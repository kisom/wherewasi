#[macro_use]
extern crate serde_derive;

pub mod location;

fn main() {
    let oakland = location::core::Coordinates::new(37.8044, -122.2711, 13.0, 2000.0, 100.0);
    println!("{:?}", oakland);
    println!("{}", oakland);
}
