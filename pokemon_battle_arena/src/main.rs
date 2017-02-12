#![allow(dead_code)]
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate conrod;
extern crate rustc_serialize;
extern crate time;
extern crate piston_window;

mod arena;
mod db;
mod graphic;
mod player;

use player::Player;
use player::PlayerType;
use arena::Arena;

fn main() {
    println!("");
    println!("Hello and Welcome to the Pokemon Battle Arena");
    println!("");
    testing();
}

// This function is for testing. Pls uncommend before commiting!
fn testing() {
    println!("Testing:");
    // for entry in db::movedex::Movedex::new().get_entries() {
    //     if entry.get_category() == enums::Move_Category::Swagger {
    //         println!("{:?}", entry.get_name());
    //     }
    // }
    // test_players();
    // test_arena();
    // graphic::gui::draw_window();
}



fn test_players() {

    println!("The Player Section");
    let mut human = Player::new_by_id(&[5, 3, 17], PlayerType::Human);

    println!("Creating a player and testing count and getting the name");
    println!("Count: {}", human.get_pokemon_count());
    println!("Place 1: {}", human.get_pokemon_list()[0].get_name());
    println!("Place 2: {}", human.get_pokemon_list()[1].get_name());
    println!("Place 3: {}", human.get_pokemon_list()[2].get_name());

    println!("Alive list:");
    println!("Number: {}", human.get_alive_count());
    println!("List: {:#?}", human.get_alive_list());

    println!("Current:");
    println!("Default: {}", human.get_current());
    human.set_current(2);
    println!("Custom: {}", human.get_current());
}

fn test_arena() {
    let human1 = Player::new_by_id(&[5, 3, 17], PlayerType::Human);
    let human2 = Player::new_by_id(&[18, 19, 122], PlayerType::Human);
    let arena = Arena::new(human1, human2,
        db::enums::Types::Normal, db::enums::Weather::ClearSky);
    println!("{}", arena.get_player_one().get_pokemon_count());
    println!("{}", arena.get_player_two().get_pokemon_count());
    println!("{:#?}", arena.get_player_one().get_pokemon_list());
    println!("{:#?}", arena.get_player_two().get_pokemon_list());
    println!("{:?}", arena.get_weather());
}
