extern crate csv;
extern crate num;
extern crate rustc_serialize;

use super::pokemon_model;
use self::num::FromPrimitive;

///contains all important information of a move
#[derive(Debug, RustcDecodable, Clone)]
pub struct Technique {
    attack_id: u16,
    name: String,
    attack_type: pokemon_model::types,
    power: Option<u16>,
    power_points: Option<u8>,
    accuracy: Option<u16>,
    has_priority: bool,
    target: Target,
    damage_class: DamageClass,
    effect_id: u16,
    effect_chance: u8,
}

///temporary struct to read out of a csv file
///Is needed, because reading into a tuple isn't possible because of the number of columns
#[derive(Debug, RustcDecodable)]
pub struct TechniqueTmp {
    attack_id: u16,
    name: String,
    generation: u8,
    attack_type: i32,
    power: Option<u16>,
    power_points: Option<u8>,
    accuracy: Option<u16>,
    has_priority: Option<u8>,
    target: i32,
    damage_class: i32,
    effect: u16,
    effect_chance: Option<u8>,
    contest_one: Option<usize>,
    contest_two: Option<usize>,
    contest_three: Option<usize>,
}

///enum for the Damage Class of a attack.
///Can be assigned from a i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone)]
    enum DamageClass {
        Physical = 1,
        Special = 2,
        Status = 3,
    }
}

///Enum that contains the valid target(s) of a move.
///Can be assigned from a i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone)]
    enum Target {
        SpecificMove = 1,
        SelectedPokemonMeFirst = 2,
        Ally = 3,
        UsersField = 4,
        UserOrAlly = 5,
        OpponentsField = 6,
        User = 7,
        RandomOpponent = 8,
        AllOtherPokemon = 9,
        SelectedPokemon = 10,
        AllOpponents = 11,
        EntireField = 12,
        UserAndAllies = 13,
        AllPokemon = 14,
    }
}

///creates similar to the pokedex a Vec that contains all known moves.
pub fn create_movedex() -> Vec<Technique> {
    let mut moves = Vec::new();
    let mut move_db = csv::Reader::from_file("./src/db/tables/moves.csv").unwrap();
    for record in move_db.decode() {
        let tmp: TechniqueTmp = record.unwrap();
        let chance = match tmp.effect_chance {
            Some(n) => n,
            None => 100,
        };
        if tmp.attack_id < 622 {
            let attack = Technique {
                attack_id: tmp.attack_id,
                name: tmp.name,
                attack_type: pokemon_model::types::from_i32(tmp.attack_type).
                unwrap_or(pokemon_model::types::undefined),
                power: tmp.power,
                power_points: tmp.power_points,
                accuracy: tmp.accuracy,
                has_priority: { tmp.has_priority == Some(1) },
                target: Target::from_i32(tmp.target).unwrap(),
                damage_class: DamageClass::from_i32(tmp.damage_class).unwrap(),
                effect_id: tmp.effect,
                effect_chance: chance,
            };
            moves.push(attack);
        }
    }
    moves
}

pub fn move_by_id(id: usize) -> Option<Technique> {
    let movedex = create_movedex();
    if id < 622 {
        return Some(movedex[id - 1].clone());
    }
    None
}
