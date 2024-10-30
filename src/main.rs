use soi::config::load_config;
mod config;

fn main() {
    println!("Hello, world!");
    load_config().expect("TODO: panic message");
}