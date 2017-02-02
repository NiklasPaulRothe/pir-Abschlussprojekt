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
use arena::Arena;
use db::enums;

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
    // graphic::windows::draw_window();
    // test_players();
    // test_arena()
}



fn test_players() {

    println!("The Player Section");
    let mut human = player::human::Human::new_by_id(&[5, 3, 17]);

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
    let human1 = player::human::Human::new_by_id(&[5, 3, 17]);
    let human2 = player::human::Human::new_by_id(&[18, 19, 122]);
    let arena = Arena::new(vec![Box::new(human1)], vec![Box::new(human2)],
        db::enums::types::normal, db::enums::Weather::Clear_Sky);
    println!("{}", arena.get_team_1_player(1).unwrap().get_pokemon_count());
    println!("{}", arena.get_team_2_player(1).unwrap().get_pokemon_count());
    println!("{:#?}", arena.get_team_1_player(1).unwrap().get_pokemon_list());
    println!("{:#?}", arena.get_team_2_player(1).unwrap().get_pokemon_list());
    println!("{:?}", arena.get_weather());
}
