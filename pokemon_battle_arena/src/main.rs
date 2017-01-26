#[macro_use] extern crate enum_primitive;
extern crate rustc_serialize;
extern crate time;
extern crate piston_window;

mod arena;
mod db;
mod graphic;
mod player;

use time::get_time;
use player::Player;

fn main() {
    println!("");
    println!("Hello and Welcome to the Pokemon Battle Arena");
    println!("");
    //test_players();
}

fn test_players() {
    let human = player::human::Human::new_by_id(&[5, 3]);

    println!("{}", human.get_pokemon_list()[0].name);
    println!("{}", human.get_pokemon_list()[1].name);
}
