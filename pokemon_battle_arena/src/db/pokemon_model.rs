use super::enums;
use super::stats;

use enum_primitive::FromPrimitive;
use std::borrow::BorrowMut;

///Basic values for Pokemon species. Equal for every instance of the given Pokemon.
#[derive(Debug, Clone)]
pub struct PokemonModel {
    pokedex_id: usize,
    name: String,
    type_one: enums::types,
    type_two: enums::types,
    base_stats: stats::Stats,
    mega_evolution: Box<Option<PokemonModel>>,
}

impl PokemonModel {
    pub fn new(id: usize, name: String) -> PokemonModel {
        PokemonModel {
                    pokedex_id: id,
                    name: name,
                    type_one: enums::types::from_i32(19).unwrap(),
                    type_two: enums::types::from_i32(19).unwrap(),
                    base_stats: stats::Stats::default(),
                    mega_evolution: Box::new(None),
                }
    }

    pub fn get_id(&self) -> usize {
        self.pokedex_id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_types(&self) -> (enums::types, enums::types) {
        (self.clone().type_one, self.clone().type_two)
    }

    pub fn get_stats(&self) -> stats::Stats {
        self.clone().base_stats
    }

    pub fn get_mega(&self) -> Option<PokemonModel> {
        if self.has_mega() {
           return Some(self.clone().mega_evolution.unwrap());
        }
        None
    }

    pub fn has_mega(&self) -> bool {
        if self.mega_evolution.is_some() {
            return true;
        }
        false
    }

    pub fn set_type(&mut self, type_id: i32, slot: u16) {
        match slot {
            1 => self.type_one = enums::types::from_i32(type_id).unwrap(),
            2 => self.type_two = enums::types::from_i32(type_id).unwrap(),
            _ => unreachable!(),
        }
    }

    pub fn set_mega(&mut self, model: PokemonModel) {
        self.mega_evolution = Box::new(Some(model));
    }

    pub fn set_stats(&mut self, stat_id: i32, value: u16) {
        self.base_stats.set_stats(enums::Stats::from_i32(stat_id).unwrap(), value);
    }
}
