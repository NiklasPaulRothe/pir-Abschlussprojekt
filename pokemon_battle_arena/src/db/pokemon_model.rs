extern crate num;
extern crate csv;
extern crate rustc_serialize;
extern crate regex;

use self::num::FromPrimitive;
use self::regex::Regex;
use std::collections::HashMap;

enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone)]
    pub enum types {
        normal = 1,
        fighting = 2,
        flying = 3,
        poison = 4,
        ground = 5,
        rock = 6,
        bug = 7,
        ghost = 8,
        steel = 9,
        fire = 10,
        water = 11,
        grass = 12,
        electric = 13,
        psychic = 14,
        ice = 15,
        dragon = 16,
        dark = 17,
        fairy = 18,
        undefined = 19,
    }
}

#[derive(Debug, Clone)]
pub struct PokemonModel {
    pokedex_id: usize,
    name: String,
    type_one: types,
    type_two: types,
    hp: u16,
    attack: u16,
    defense: u16,
    special_attack: u16,
    special_defense: u16,
    speed: u16,
    pub mega_evolution: Box<Option<PokemonModel>>,
}

///creates a pokedex with all known Pokemon
pub fn create_pokedex() -> Vec<PokemonModel> {
    let mut pokemon = Vec::new();
    let mut pokemon_db = csv::Reader::from_file("./src/db/tables/pokemon.csv").unwrap();
    let re = Regex::new(r"mega").unwrap();
    let mut mega = HashMap::new();
    for record in pokemon_db.decode() {
        let(id, name, species, _, _, _, _, _): (usize, String, usize, usize, usize, usize, usize, usize)
            = record.unwrap();
        let re = Regex::new(r"mega").unwrap();
        if id < 722 {
            pokemon.push(PokemonModel {
                pokedex_id: id,
                name: name,
                type_one: types::from_i32(19).unwrap(),
                type_two: types::from_i32(19).unwrap(),
                hp: 0,
                attack: 0,
                defense: 0,
                special_attack: 0,
                special_defense: 0,
                speed: 0,
                mega_evolution: Box::new(None),
            });
        }
        else if id > 10000 && re.is_match(&name) {
            pokemon[species - 1].mega_evolution = Box::new(Some(PokemonModel {
                pokedex_id: id,
                name: name,
                type_one: types::from_i32(19).unwrap(),
                type_two: types::from_i32(19).unwrap(),
                hp: 0,
                attack: 0,
                defense: 0,
                special_attack: 0,
                special_defense: 0,
                speed: 0,
                mega_evolution: Box::new(None),
            }));
            mega.insert(id, species);
        }
    }
    let mut type_db = csv::Reader::from_file("./src/db/tables/pokemon_types.csv").unwrap();
    for record in type_db.decode() {
        let(poke_id, type_id, slot): (usize, i32, u16) = record.unwrap();
        if poke_id < 722 {
            match slot {
                1 => pokemon[poke_id - 1].type_one = types::from_i32(type_id).unwrap(),
                2 => pokemon[poke_id - 1].type_two = types::from_i32(type_id).unwrap(),
                _ => unreachable!(),
            }
        }
        else if poke_id > 10000 && mega.contains_key(&poke_id) {
            match slot {
                1 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.type_one = types::from_i32(type_id).unwrap();
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                }
                2 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.type_two = types::from_i32(type_id).unwrap();
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                }
                _ => unreachable!(),
            }
        }
    }
    let mut stat_db = csv::Reader::from_file("./src/db/tables/pokemon_stats.csv").unwrap();
    for record in stat_db.decode() {
        let(poke_id, stat_id, stat, _): (usize, u8, u16, u8) = record.unwrap();
        if poke_id < 722 {
            match stat_id {
                1 => pokemon[poke_id - 1].hp = stat,
                2 => pokemon[poke_id - 1].attack = stat,
                3 => pokemon[poke_id - 1].defense = stat,
                4 => pokemon[poke_id - 1].special_attack = stat,
                5 => pokemon[poke_id - 1].special_defense = stat,
                6 => pokemon[poke_id - 1].speed = stat,
                _ => unreachable!(),
            }
        }
        else if poke_id > 10000 && mega.contains_key(&poke_id) {
            match stat_id {
                1 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.hp = stat;
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                },
                2 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.attack = stat;
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                },
                3 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.defense = stat;
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                },
                4 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.special_attack = stat;
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                },
                5 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.special_defense = stat;
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                },
                6 => {
                    let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                    mega_evolution.unwrap();
                    poke.speed = stat;
                    pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                    Box::new(Some(poke));
                },
                _ => unreachable!(),
            }
        }
    }
    pokemon
}

pub fn pokemon_by_id(id: usize) -> Option<PokemonModel> {
    let pokedex = create_pokedex();
    if id < 722 {
        return Some(pokedex[id - 1].clone());
    }
    None
}

pub fn pokemon_by_name(name: String, pokedex: Vec<PokemonModel>) -> Option<PokemonModel> {
    for entry in pokedex {
        if entry.name == name {
            return Some(entry)
        }
    }
    None
}
