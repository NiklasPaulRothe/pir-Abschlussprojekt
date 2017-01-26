extern crate csv;
extern crate num;
extern crate rustc_serialize;

use super::pokemon_model;
use super::pokemon_token;
use super::enums;
use self::num::FromPrimitive;
use std::collections::HashMap;
use std;

///contains all important information of a move
#[derive(Debug, Clone)]
pub struct Technique {
    attack_id: usize,
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
    description: String,
}

///temporary struct to read out of a csv file
///Is needed, because reading into a tuple isn't possible because of the number of columns
#[derive(Debug, RustcDecodable, Clone)]
pub struct TechniqueTmp {
    pub attack_id: usize,
    pub name: String,
    generation: u8,
    pub attack_type: i32,
    pub power: Option<u16>,
    pub power_points: Option<u8>,
    pub accuracy: Option<u16>,
    pub priority: Option<u8>,
    pub target: i32,
    pub damage_class: i32,
    pub effect: u16,
    pub effect_chance: Option<u8>,
    contest_one: Option<usize>,
    contest_two: Option<usize>,
    contest_three: Option<usize>,
}




impl Technique {
    pub fn from_tmp(tmp: TechniqueTmp, effective_hash: HashMap<enums::types, i8>,
        description: String) -> Technique {
        let chance = match tmp.effect_chance {
                Some(n) => n,
                None => 100,
        };

        Technique {
            attack_id: tmp.attack_id,
            name: tmp.name,
            attack_type: enums::types::from_i32(tmp.attack_type).
            unwrap_or(enums::types::undefined),
            power: tmp.power,
            power_points: tmp.power_points,
            accuracy: tmp.accuracy,
            priority: { tmp.priority == Some(1) },
            target: enums::Target::from_i32(tmp.target).unwrap(),
            typeeffectiveness: effective_hash,
            damage_class: enums::DamageClass::from_i32(tmp.damage_class).unwrap(),
            effect_id: tmp.effect,
            effect_chance: chance,
            description: description,
        }
    }

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

    pub fn get_id(&self) -> usize {
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

    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}
