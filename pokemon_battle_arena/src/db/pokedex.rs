extern crate csv;
extern crate rustc_serialize;
extern crate regex;

use std::collections::HashMap;
use self::regex::Regex;
use enum_primitive::FromPrimitive;

use super::pokemon_model::PokemonModel;
use super::stats;
use super::enums;

///Pokedex struct that is used to get an overview over the possible Pokemon. Besides the entries
///Vector with PokemonModels inside, it contains a bool variable which tells if the pokedex contains
///every known Pokemon or not.
pub struct Pokedex {
    entries: Vec<PokemonModel>,
    complete: bool,
}

impl Pokedex {
    ///returns a pokemon from it's pokedex number
    pub fn pokemon_by_id(&self, id: usize) -> Option<PokemonModel> {
        if id < 722 && self.is_complete() {
            return Some(self.entries[id - 1].clone());
        }
        else if id < 722 {
            for entry in self.entries.clone() {
                if entry.pokedex_id == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    ///returns a pokemon from it's name
    pub fn pokemon_by_name(&self, name: String) -> Option<PokemonModel> {
        for entry in self.entries.clone() {
            if entry.name == name {
            return Some(entry)
        }
    }
    None
    }

    pub fn get_entries(&self) -> Vec<PokemonModel> {
        self.entries.clone()
    }

    fn is_complete(&self) -> bool {
        self.complete
    }


    ///creates a pokedex with all known Pokemon
    pub fn new() -> Pokedex {
        let mut pokemon = Vec::new();
        let re = Regex::new(r"mega").unwrap();
        let mut mega = HashMap::new();

        //creates the basic pokemon model with pokedex ID and name.
        let mut pokemon_db = csv::Reader::from_file("./src/db/tables/pokemon.csv").unwrap();
        for record in pokemon_db.decode() {
            let(id, name, species, _, _, _, _, _):
            (usize, String, usize, usize, usize, usize, usize, usize) = record.unwrap();
            let re = Regex::new(r"mega").unwrap();
            if id < 722 {
                pokemon.push(PokemonModel {
                    pokedex_id: id,
                    name: name,
                    type_one: enums::types::from_i32(19).unwrap(),
                    type_two: enums::types::from_i32(19).unwrap(),
                    base_stats: stats::Stats {
                        hp: 0,
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                    },
                    mega_evolution: Box::new(None),
                });
            }
            //adds mega evolutions if available
            else if id > 10000 && re.is_match(&name) {
                pokemon[species - 1].mega_evolution = Box::new(Some(PokemonModel {
                    pokedex_id: id,
                    name: name,
                    type_one: enums::types::from_i32(19).unwrap(),
                    type_two: enums::types::from_i32(19).unwrap(),
                    base_stats: stats::Stats {
                        hp: 0,
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                    },
                    mega_evolution: Box::new(None),
                }));
                //saves where to find the mega evolutions by their ID
                mega.insert(id, species);
            }
        }

        //adds types to the constructed pokemon models
        let mut type_db = csv::Reader::from_file("./src/db/tables/pokemon_types.csv").unwrap();
        for record in type_db.decode() {
            let(poke_id, type_id, slot): (usize, i32, u16) = record.unwrap();
            if poke_id < 722 {
                match slot {
                    1 => pokemon[poke_id - 1].type_one = enums::types::from_i32(type_id).unwrap(),
                    2 => pokemon[poke_id - 1].type_two = enums::types::from_i32(type_id).unwrap(),
                    _ => unreachable!(),
                }
            }
            else if poke_id > 10000 && mega.contains_key(&poke_id) {
                match slot {
                    1 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.type_one = enums::types::from_i32(type_id).unwrap();
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    }
                    2 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.type_two = enums::types::from_i32(type_id).unwrap();
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    }
                    _ => unreachable!(),
                }
            }
        }

        //adds the base stats to every Pokemon Model
        let mut stat_db = csv::Reader::from_file("./src/db/tables/pokemon_stats.csv").unwrap();
        for record in stat_db.decode() {
            let(poke_id, stat_id, stat, _): (usize, u8, u16, u8) = record.unwrap();
            if poke_id < 722 {
                match stat_id {
                    1 => pokemon[poke_id - 1].base_stats.hp = stat,
                    2 => pokemon[poke_id - 1].base_stats.attack = stat,
                    3 => pokemon[poke_id - 1].base_stats.defense = stat,
                    4 => pokemon[poke_id - 1].base_stats.special_attack = stat,
                    5 => pokemon[poke_id - 1].base_stats.special_defense = stat,
                    6 => pokemon[poke_id - 1].base_stats.speed = stat,
                    _ => unreachable!(),
                }
            }
            else if poke_id > 10000 && mega.contains_key(&poke_id) {
                match stat_id {
                    1 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.base_stats.hp = stat;
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    },
                    2 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.base_stats.attack = stat;
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    },
                    3 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.base_stats.defense = stat;
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    },
                    4 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.base_stats.special_attack = stat;
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    },
                    5 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.base_stats.special_defense = stat;
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    },
                    6 => {
                        let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                        mega_evolution.unwrap();
                        poke.base_stats.speed = stat;
                        pokemon[(mega.get(&poke_id)).unwrap() - 1].mega_evolution =
                        Box::new(Some(poke));
                    },
                    _ => unreachable!(),
                }
            }
        }
        Pokedex {
            entries: pokemon,
            complete: true,
        }
    }
}
