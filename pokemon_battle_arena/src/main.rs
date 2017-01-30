#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate conrod;
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
    testing();
}

// This function is for testing. Pls uncommend before commiting!
fn testing() {
    println!("Testing:");
    // for entry in db::pokedex::Pokedex::new().get_entries() {
    //     println!("{:?}", entry);
    // }
    // graphic::windows::draw_startscreen();
    // test_players();
}



fn test_players() {

    println!("The Player Section");
    let human = player::human::Human::new_by_id(&[5, 3, 17]);

    println!("{}", human.get_pokemon_count());
    println!("{}", human.get_pokemon_list()[0].get_name());
    println!("{}", human.get_pokemon_list()[1].get_name());
    println!("{}", human.get_pokemon_list()[2].get_name());
    println!("{}", human.get_alive());
}
