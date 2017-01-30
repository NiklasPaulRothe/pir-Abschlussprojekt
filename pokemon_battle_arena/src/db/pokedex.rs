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
                if entry.get_id() == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    ///returns a pokemon from it's name
    pub fn pokemon_by_name(&self, name: String) -> Option<PokemonModel> {
        for entry in self.entries.clone() {
            if entry.get_name() == name {
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
                pokemon.push(PokemonModel::new(id, name));
            }
            //adds mega evolutions if available
            else if id > 10000 && re.is_match(&name) {
                pokemon[species - 1].set_mega(PokemonModel::new(id, name));
                //saves where to find the mega evolutions by their ID
                mega.insert(id, species);
            }
        }

        //adds types to the constructed pokemon models
        let mut type_db = csv::Reader::from_file("./src/db/tables/pokemon_types.csv").unwrap();
        for record in type_db.decode() {
            let(poke_id, type_id, slot): (usize, i32, u16) = record.unwrap();
            if poke_id < 722 {
                pokemon[poke_id - 1].set_type(type_id, slot);
            }
            else if poke_id > 10000 && mega.contains_key(&poke_id) {
                let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                get_mega().unwrap();
                poke.set_type(type_id, slot);
                pokemon[(mega.get(&poke_id)).unwrap() - 1].set_mega(poke);
            }
        }

        //adds the base stats to every Pokemon Model
        let mut stat_db = csv::Reader::from_file("./src/db/tables/pokemon_stats.csv").unwrap();
        for record in stat_db.decode() {
            let(poke_id, stat_id, stat, _): (usize, i32, u16, u8) = record.unwrap();
            if poke_id < 722 {
                pokemon[poke_id - 1].set_stats(stat_id, stat);
            }
            else if poke_id > 10000 && mega.contains_key(&poke_id) {
                let mut poke = pokemon[(mega.get(&poke_id)).unwrap() - 1].clone().
                get_mega().unwrap();
                poke.set_stats(stat_id, stat);
                pokemon[(mega.get(&poke_id)).unwrap() - 1].set_mega(poke);

            }
        }
        Pokedex {
            entries: pokemon,
            complete: true,
        }
    }
}
