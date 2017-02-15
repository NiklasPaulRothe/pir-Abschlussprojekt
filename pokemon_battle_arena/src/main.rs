#![allow(dead_code)]
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate conrod;
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
    for entry in db::movedex::Movedex::new().get_entries() {
        if entry.get_category() == enums::MoveCategory::WholeFieldEffect {
            println!("{:?}", entry.get_name());
        }
    }
    // test_players();
    test_arena();
    // let mut window = graphic::gui::App::new();
    // window.draw_window();
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
    // Arena erstellen
    let mut p1 = Player::new_by_id(&[5], PlayerType::Human);
    let mut p2 = Player::new_by_id(&[8], PlayerType::Human);
    let mut arena = Arena::new(&mut p1,
                               &mut p2,
                               db::enums::Types::Normal,
                               db::enums::Weather::ClearSky);
    // println!("Player One: {:#?}", arena.get_player_one());
    // println!("Player Two: {:#?}", arena.get_player_two());
    // Attacke erstellen und "Kampf"
    let movedex = db::movedex::Movedex::new();
    let attack = movedex.move_by_id(1).unwrap();
    let attack_haze = movedex.move_by_id(114).unwrap();
    let attack_hail = movedex.move_by_id(258).unwrap();
    let attack_sandstorm = movedex.move_by_id(201).unwrap();
    let attack_mudsport = movedex.move_by_id(300).unwrap();
    let attack_fairylock = movedex.move_by_id(587).unwrap();
    println!("Attack: {}", attack_hail.get_name());
    println!("HP1 vorher: {}",
             arena.get_player_one().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    println!("HP2 vorher: {}",
             arena.get_player_two().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    attack.resolve(&mut arena, enums::Player::Two);
    attack_hail.resolve(&mut arena, enums::Player::Two);
    attack_haze.resolve(&mut arena, enums::Player::Two);
    attack_sandstorm.resolve(&mut arena, enums::Player::Two);
    attack_mudsport.resolve(&mut arena, enums::Player::Two);
    attack_fairylock.resolve(&mut arena, enums::Player::Two);
    attack.resolve(&mut arena, enums::Player::Two);

    println!("HP1 nachher: {}",
             arena.get_player_one().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    println!("HP2 nachher: {}",
             arena.get_player_two().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    println!("Hash vorher: {:?}", arena.get_field_effects());
    println!("Weather vorher: {:?}", arena.get_current_weather());
    arena.validate_effects_and_weather();
    arena.validate_effects_and_weather();
    println!("Hash nach 2 Runden: {:?}", arena.get_field_effects());
    println!("Weather nach 2 Runden: {:?}", arena.get_current_weather());
    arena.validate_effects_and_weather();
    arena.validate_effects_and_weather();
    arena.validate_effects_and_weather();
    println!("Hash nach 5 Runden: {:?}", arena.get_field_effects());
    println!("Weather nach 5 Runden: {:?}", arena.get_current_weather());
    arena.validate_effects_and_weather();
    arena.validate_effects_and_weather();
    println!("Hash nach 7 Runden: {:?}", arena.get_field_effects());
    println!("Weather nach 7 Runden: {:?}", arena.get_current_weather());
    // println!("Player One: {:#?}", arena.get_player_one());
}
