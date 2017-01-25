#[macro_use] extern crate enum_primitive;
extern crate rustc_serialize;
extern crate time;

use time::get_time;

mod arena;
mod db;
mod graphic;
mod player;

fn main() {
    let pokedex = db::pokemon_model::create_pokedex();
    let ivysaur = db::pokemon_token::PokemonToken::from_model(pokedex[2].clone());
    println!("{:?}", ivysaur);
    println!("{:?}", ivysaur.get_mega());
}
