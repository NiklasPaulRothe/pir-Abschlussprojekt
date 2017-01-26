extern crate csv;
extern crate num;
extern crate rustc_serialize;

use super::pokemon_model;
use super::pokemon_token;
use super::enums;
use self::num::FromPrimitive;
use std::collections::HashMap;

///contains all important information of a move
#[derive(Debug, RustcDecodable, Clone)]
pub struct Technique {
    attack_id: u16,
    name: String,
    attack_type: enums::types,
    power: Option<u16>,
    power_points: Option<u8>,
    accuracy: Option<u16>,
    priority: bool,
    target: enums::Target,
    typeeffectiveness: HashMap<enums::types, i8>,
    damage_class: enums::DamageClass,
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
    priority: Option<u8>,
    target: i32,
    damage_class: i32,
    effect: u16,
    effect_chance: Option<u8>,
    contest_one: Option<usize>,
    contest_two: Option<usize>,
    contest_three: Option<usize>,
}




impl Technique {
    pub fn get_effectiveness(&self, enemy: pokemon_token::PokemonToken) -> enums::TypeEffectiveness {
        let mut eff_count = 0;
        if self.typeeffectiveness.contains_key(&enemy.type_one) {
            eff_count = eff_count + self.typeeffectiveness.get(&enemy.type_one).unwrap();
        }
        if enemy.type_two != enums::types::undefined
        && self.typeeffectiveness.contains_key(&enemy.type_one) {
            eff_count = eff_count + self.typeeffectiveness.get(&enemy.type_two).unwrap();
        }
        match eff_count {
            -2 => enums::TypeEffectiveness::NotEffective,
            -1 => enums::TypeEffectiveness::NotVeryEffective,
            0 => enums::TypeEffectiveness::Normal,
            1 => enums::TypeEffectiveness::VeryEffective,
            2 => enums::TypeEffectiveness::SuperEffective,
            _ => enums::TypeEffectiveness::Ineffective,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.attack_id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_type(&self) -> enums::types {
        self.attack_type.clone()
    }

    pub fn get_power(&self) -> Option<u16> {
        self.power
    }

    pub fn get_power_points(&self) -> Option<u8> {
        self.power_points
    }

    pub fn get_accuracy(&self) -> Option<u16> {
        self.accuracy
    }

    pub fn has_priority(&self) -> bool {
        self.priority
    }

    pub fn get_target(&self) -> enums::Target {
        self.target.clone()
    }

    pub fn get_damage_class(&self) -> enums::DamageClass {
        self.damage_class.clone()
    }

    pub fn get_effect_chance(&self) -> u8 {
        self.effect_chance
    }
}
