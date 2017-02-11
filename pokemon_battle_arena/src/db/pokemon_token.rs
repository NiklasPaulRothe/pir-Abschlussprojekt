extern crate csv;

use super::pokemon_model;
use super::natures;
use super::enums;
use super::stats;
use super::determinant_values;
use super::movedex;
use super::moves;

use std::collections::HashMap;

///Represents a single Token of a Pokemon with individual values for this token.
#[derive(Debug, Clone)]
pub struct PokemonToken {
    pokedex_id: usize,
    name: String,
    level: u16,
    height: u8,
    weight: u16,
    gender: enums::Gender,
    type_one: enums::Types,
    type_two: enums::Types,
    non_volatile_status: (enums::NonVolatile, u8),
    move_one: Option<(moves::Technique, u8)>,
    move_two: Option<(moves::Technique, u8)>,
    move_three: Option<(moves::Technique, u8)>,
    move_four: Option<(moves::Technique, u8)>,
    nature: natures::Nature,
    dv: determinant_values::Dv,
    base_stats: stats::Stats,
    current_stats: stats::Stats,
    end_of_turn_flags: HashMap<enums::EndOfTurn, u8>,
    description: String,
    mega_evolution: Option<pokemon_model::PokemonModel>,
}


impl PokemonToken {
    ///Provides a Pokemon Token from a given model.
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        let level = 50;
        let dv = determinant_values::Dv::get_dvs(model.clone());
        let nature = natures::Nature::get_random_nature();
        let stats = stats::Stats::calculate_stats(model.clone(), dv.clone(), nature.clone(), level);

        PokemonToken {
            pokedex_id: model.get_id(),
            name: model.get_name(),
            level: level,
            height: model.get_height(),
            weight: model.get_weight(),
            gender: enums::get_gender(model.clone().get_gender_rate()),
            type_one: model.get_types().0,
            type_two: model.get_types().1,
            non_volatile_status: (enums::NonVolatile::Undefined, 0),
            move_one: None,
            move_two: None,
            move_three: None,
            move_four: None,
            nature: nature,
            dv: dv,
            base_stats: stats.clone(),
            current_stats: stats,
            end_of_turn_flags: HashMap::new(),
            description: model.get_description(),
            mega_evolution: model.get_mega(),
        }
    }
    pub fn is_asleep(&self) -> bool {
        self.non_volatile_status.0 == enums::NonVolatile::Sleep
    }
    pub fn get_moves(&self, dex: movedex::Movedex) -> movedex::Movedex {
        dex.for_token(self.get_level(), self.pokedex_id)
    }
    pub fn get_id(&self) -> usize {
        self.pokedex_id
    }
    pub fn get_name(&self) -> String {
        self.clone().name
    }
    pub fn get_level(&self) -> u16 {
        self.clone().level
    }
    pub fn get_gender(&self) -> enums::Gender {
        self.clone().gender
    }
    pub fn get_types(&self) -> (enums::Types, enums::Types) {
        (self.clone().type_one, self.clone().type_two)
    }
    pub fn get_nature(&self) -> natures::Nature {
        self.clone().nature
    }
    pub fn get_non_volatile(&self) ->(enums::NonVolatile, u8) {
        self.clone().non_volatile_status
    }
    pub fn get_dv(&self) -> determinant_values::Dv {
        self.clone().dv
    }
    pub fn get_current(&self) -> stats::Stats {
        self.current_stats.clone()
    }
    pub fn get_base(&self) -> stats::Stats {
        self.base_stats.clone()
    }
    pub fn get_end_of_turn_flags(&self) -> HashMap<enums::EndOfTurn, u8> {
        self.clone().end_of_turn_flags
    }

    pub fn get_description(&self) -> String {
        self.clone().description
    }

    pub fn get_mega(&self) -> Option<PokemonToken> {
        if self.mega_evolution.is_some() {
            return Some(PokemonToken::from_model(self.mega_evolution.clone().unwrap()));
        }
        None
    }

    /// Getter function for move one. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_one(self) -> Option<moves::Technique> {
        if let Some(x) = self.move_one {
            return Some(x.0);
        }
        None
    }
    /// Getter function for move two. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_two(self) -> Option<moves::Technique> {
        if let Some(x) = self.move_two {
            return Some(x.0);
        }
        None
    }
    /// Getter function for move three. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_three(self) -> Option<moves::Technique> {
        if let Some(x) = self.move_three {
            return Some(x.0);
        }
        None
    }
    /// Getter function for move four. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_four(self) -> Option<moves::Technique> {
        if let Some(x) = self.move_four {
            return Some(x.0);
        }
        None
    }

    pub fn set_non_volatile(&mut self, status: enums::NonVolatile) {
        self.non_volatile_status = (status, 0);
    }
    pub fn add_end_flag(&mut self, flag: enums::EndOfTurn) {
        self.end_of_turn_flags.insert(flag, 0);
    }
    pub fn set_moves(&mut self, moves: Vec<moves::Technique>) {
        self.move_one = Some((moves[0].clone(), moves[0].get_power_points().unwrap()));
        if moves.len() >= 1 {
            self.move_two = Some((moves[1].clone(), moves[1].get_power_points().unwrap()));
        }
        if moves.len() >= 2 {
            self.move_three = Some((moves[2].clone(), moves[2].get_power_points().unwrap()));
        }
        if moves.len() >= 3 {
            self.move_four = Some((moves[3].clone(), moves[3].get_power_points().unwrap()));
        }
    }
    pub fn set_type(&mut self, position: u8, change: enums::Types) {
        match position {
            0 => self.type_one = change,
            1 => self.type_two = change,
            _ => unreachable!(),
        }
    }
    pub fn decrement_ap(&mut self) {
        unimplemented!();
    }
}
