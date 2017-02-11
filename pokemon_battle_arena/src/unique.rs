use time::get_time;
use player::Player;
use player::PlayerType;
use arena::Arena;
use db;
use db::enums;
use db::moves;
use db::pokemon_token::PokemonToken;
use db::movedex::Movedex;

// extern crate rand;
// extern crate regex;

// use super::moves::Technique;
// use super::pokemon_token::PokemonToken;
// use super::enums;
// use self::rand::{Rng, thread_rng};
// use self::regex::Regex;
// use player::Player;

pub fn test(){
    println!("Unique");
    for entry in Movedex::new().get_entries() {
         if entry.get_category() == enums::Move_Category::Unique {
             println!("{:?}: {:?} - {:?}", entry.get_id(), entry.get_name(), entry.get_type());
         }
    }

}

pub fn unique(name: String, move_type: enums::types, ailment: enums::Ailment, mut target: PokemonToken) {
    target.get_moves(Movedex);
}
