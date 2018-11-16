#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod location;

fn main() {

    let database_file = location::db::get_database();
    println!("configure database: {}", database_file);

    let oakland = location::core::Coordinates::new(37.8044, -122.2711, 13.0, 2000.0, 100.0);
    let oakland_json = serde_json::to_string(&oakland).unwrap();
    println!("{}", oakland_json);
}
