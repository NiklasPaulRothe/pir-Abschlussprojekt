extern crate csv;
extern crate num;
extern crate rustc_serialize;

use super::pokemon;
use self::num::FromPrimitive;

#[derive(Debug, RustcDecodable)]
pub struct Technique {
    attack_id: u16,
    name: String,
    attack_type: pokemon::types,
    power: Option<u16>,
    power_points: Option<u8>,
    accuracy: Option<u16>,
    has_priority: bool,
    target: Target,
    damage_class: DamageClass,
    effect_id: u16,
    effect_chance: Option<u8>,
}

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

enum_from_primitive! {
    #[derive(Debug, RustcDecodable)]
    enum DamageClass {
        Physical = 1,
        Special = 2,
        Status = 3,
    }
}

enum_from_primitive! {
    #[derive(Debug, RustcDecodable)]
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

pub fn create_movedex() -> Vec<Technique> {
    let mut moves = Vec::new();
    let mut move_db = csv::Reader::from_file("./src/db/tables/moves.csv").unwrap();
    for record in move_db.decode() {
        let tmp: TechniqueTmp = record.unwrap();
        if tmp.attack_id < 622 {
            let attack = Technique {
                attack_id: tmp.attack_id,
                name: tmp.name,
                attack_type: pokemon::types::from_i32(tmp.attack_type).
                unwrap_or(pokemon::types::undefined),
                power: tmp.power,
                power_points: tmp.power_points,
                accuracy: tmp.accuracy,
                has_priority: { tmp.has_priority == Some(1) },
                target: Target::from_i32(tmp.target).unwrap(),
                damage_class: DamageClass::from_i32(tmp.damage_class).unwrap(),
                effect_id: tmp.effect,
                effect_chance: tmp.effect_chance
            };
            moves.push(attack);
        }
    }
    moves
}
