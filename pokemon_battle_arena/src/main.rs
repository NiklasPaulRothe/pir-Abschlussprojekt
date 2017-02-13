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
        if entry.get_category() == enums::MoveCategory::FieldEffect {
            println!("{:?}", entry.get_name());
        }
    }
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



fn test_arena2() {
    // let mut human1 = Player::new_by_id(&[5, 3, 17], PlayerType::Human);
    // let mut human2 = Player::new_by_id(&[18, 19, 122], PlayerType::Human);
    // let mut arena = Arena::new(&mut human1, &mut human2,
    //     db::enums::Types::Normal, db::enums::Weather::ClearSky);
    // let movedex = db::movedex::Movedex::new();
    // let attackid = 1;
    // let attack = movedex.move_by_id(attackid).unwrap();
    // // println!("{}", arena.get_player_one().get_pokemon_count());
    // // println!("{}", arena.get_player_two().get_pokemon_count());
    // println!("P 1: {:#?}", arena.get_player_one().get_pokemon_list()[0].get_name());
    // println!("Evasion: {:#?}", arena.get_player_one().get_pokemon_list()[0].get_current()
    //     .get_stat(db::enums::Stats::Evasion));
    // println!("P 2:{:#?}", arena.get_player_two().get_pokemon_list()[0].get_name());
    // println!("Attack:{:?}", attack);
    // println!("HPAnfang:{:#?}", arena.get_player_one().get_pokemon_list()[0].get_current()
    //     .get_stat(db::enums::Stats::Hp));

    // let ref mut player_one = arena.get_player_two().get_pokemon_list()[0];
    // let ref mut player_two = arena.get_player_one().get_pokemon_list()[0];
    // attack.resolve(&mut player_one,
    //     vec![&mut player_two],
    //     arena.get_player_one(),
    //     arena.get_player_two(),
    //     arena);
    // println!("HPEnde:{:#?}", arena.get_player_one().get_pokemon_list()[0].get_current()
    //     .get_stat(db::enums::Stats::Hp));

    // // println!("{:#?}", arena.get_player_two().get_pokemon_list());
    // // println!("{:?}", arena.get_weather());

}

fn test_arena() {
    // Arena erstellen
    let mut p1 = Player::new_by_id(&[5], PlayerType::Human);
    let mut p2 = Player::new_by_id(&[8], PlayerType::Human);
    let mut arena = Arena::new(&mut p1,
                               &mut p2,
                               db::enums::Types::Normal,
                               db::enums::Weather::ClearSky);
    println!("Player One: {:#?}", arena.get_player_one());
    println!("Player Two: {:#?}", arena.get_player_two());
    // Attacke erstellen und "Kampf"
    let movedex = db::movedex::Movedex::new();
    let attack = movedex.move_by_id(151).unwrap();
    println!("Attack: {}", attack.get_name());

    println!("HP1 vorher: {}",
             arena.get_player_one().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    println!("HP2 vorher: {}",
             arena.get_player_two().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));

    attack.resolve(&mut arena, 2);
    println!("HP1 nachher: {}",
             arena.get_player_one().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    println!("HP2 nachher: {}",
             arena.get_player_two().get_pokemon_list()[0]
                 .get_current()
                 .get_stat(&db::enums::Stats::Hp));
    println!("Player One: {:#?}", arena.get_player_one());

    // use std::cell::RefCell;
    // use std::rc::Rc;
    // let mut p1_clone = p1.clone();
    // let mut p2_clone = p2.clone();
    // let mut arena = Arena::new(&mut p1_clone, &mut p2_clone,
    //     db::enums::Types::Normal,
    //     db::enums::Weather::ClearSky);
    // let cell: Rc<RefCell<_>> = Rc::new(RefCell::new(Arena::new(&mut p1, &mut p2,
    //     db::enums::Types::Normal,
    //     db::enums::Weather::ClearSky)));

    // println!("Player One: {:#?}", cell.borrow_mut().get_player_one());
    // // Attacke erstellen und "Kampf"
    // let movedex = db::movedex::Movedex::new();
    // let attack = movedex.move_by_id(1).unwrap();


    // attack.resolve(&mut cell.borrow_mut().get_player_one().get_pokemon_list()[0],
    //     vec![&mut cell.borrow_mut().get_player_two().get_pokemon_list()[0]],
    //     cell.borrow_mut().get_player_one(),
    //     cell.borrow_mut().get_player_two(),
    //     &mut arena);
}
