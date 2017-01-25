extern crate num;
extern crate csv;
extern crate rustc_serialize;

use self::num::FromPrimitive;

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
pub struct pokemonModel {
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
}

///creates a pokedex with all known Pokemon
pub fn create_pokedex() -> Vec<pokemonModel> {
    let mut pokemon = Vec::new();
    let mut pokemon_db = csv::Reader::from_file("./src/db/tables/pokemon.csv").unwrap();
    for record in pokemon_db.decode() {
        let(id, name, _, _, _, _, _, _): (usize, String, usize, usize, usize, usize, usize, usize)
            = record.unwrap();
        if id < 722 {
            pokemon.push(pokemonModel {pokedex_id: id, name: name,
                type_one: types::from_i32(19).unwrap(), type_two: types::from_i32(19).unwrap(),
                hp: 0, attack: 0, defense: 0, special_attack: 0, special_defense: 0, speed: 0});
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
    }
    pokemon
}

pub fn pokemon_by_id(id: usize) -> pokemonModel {
    let pokedex = create_pokedex();
    pokedex[id - 1].clone()
}

pub fn pokemon_by_name(name: String, pokedex: Vec<pokemonModel>) -> Option<pokemonModel> {
    for entry in pokedex {
        if entry.name == name {
            return Some(entry)
        }
    }
    None
}
