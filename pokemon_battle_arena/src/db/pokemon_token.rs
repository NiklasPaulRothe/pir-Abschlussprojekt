extern crate csv;

use super::determinant_values;
use super::enums;
use super::movedex;
use super::moves;
use super::natures;
use super::pokemon_model;
use super::stats;
use std::collections::HashMap;

/// Represents a single Token of a Pokemon with individual values for this token.
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
    choose_flags: HashMap<enums::Choose, u8>,
    resolve_flags: HashMap<enums::Resolve, u8>,
    fight_flags: HashMap<enums::Fighting, u8>,
    end_of_turn_flags: HashMap<enums::EndOfTurn, u8>,
    description: String,
    mega_evolution: Option<pokemon_model::PokemonModel>,
}


impl PokemonToken {
    /// Provides a Pokemon Token from a given model.
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        let level = 50;
        let dv = determinant_values::Dv::get_dvs();
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
            choose_flags: HashMap::new(),
            resolve_flags: HashMap::new(),
            fight_flags: HashMap::new(),
            end_of_turn_flags: HashMap::new(),
            description: model.get_description(),
            mega_evolution: model.get_mega(),
        }
    }
    // Getter methods
    //
    /// Gets all possible moves the pokemon can learn
    pub fn get_moves(&self, dex: movedex::Movedex) -> movedex::Movedex {
        dex.for_token(self.get_level(), self.pokedex_id)
    }
    /// Gets the pokedex id
    pub fn get_id(&self) -> usize {
        self.pokedex_id
    }
    /// Gets the name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Gets the current level
    pub fn get_level(&self) -> u16 {
        self.level
    }
    pub fn get_weight(&self) -> u16 {
        self.weight
    }
    /// Gets the current gender
    pub fn get_gender(&self) -> enums::Gender {
        self.gender.clone()
    }
    /// Gets the current types
    pub fn get_types(&self) -> (enums::Types, enums::Types) {
        (self.type_one.clone(), self.type_two.clone())
    }
    /// Gets the current nature
    pub fn get_nature(&self) -> natures::Nature {
        self.nature.clone()
    }
    /// Gets the actual non volatile status and the amount of rounds the pokemon is holding it
    pub fn get_non_volatile(&self) -> (enums::NonVolatile, u8) {
        self.non_volatile_status.clone()
    }
    /// Gets the list of the determinant values
    pub fn get_dv(&self) -> determinant_values::Dv {
        self.dv.clone()
    }
    /// Gets the current stats
    pub fn get_current(&mut self) -> &mut stats::Stats {
        &mut self.current_stats
    }
    /// Gets the base stats for the level and all other influeces
    pub fn get_base(&self) -> stats::Stats {
        self.base_stats.clone()
    }

    pub fn get_choose_flags(&mut self) -> &mut HashMap<enums::Choose, u8> {
        &mut self.choose_flags
    }

    pub fn get_resolve_flags(&mut self) -> &mut HashMap<enums::Resolve, u8> {
        &mut self.resolve_flags
    }

    pub fn get_fight_flags(&mut self) -> &mut HashMap<enums::Fighting, u8> {
        &mut self.fight_flags
    }

    /// Gets the list of end of turn flags
    pub fn get_end_of_turn_flags(&mut self) -> &mut HashMap<enums::EndOfTurn, u8> {
        &mut self.end_of_turn_flags
    }
    /// Gets the description of the pokemon which also can be read by the user of the program
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    /// Gets the mega evolution
    pub fn get_mega(&self) -> Option<PokemonToken> {
        if self.mega_evolution.is_some() {
            return Some(PokemonToken::from_model(self.mega_evolution.clone().unwrap()));
        }
        None
    }
    /// Getter function for move one. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_one(self) -> Option<moves::Technique> {
        self.move_one.map(|x| x.0)
    }
    /// Getter function for move two. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_two(self) -> Option<moves::Technique> {
        self.move_two.map(|x| x.0)
    }
    /// Getter function for move three. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_three(self) -> Option<moves::Technique> {
        self.move_three.map(|x| x.0)
    }
    /// Getter function for move four. If the move is set, the function returns it, if not,
    /// it returns None
    pub fn get_move_four(self) -> Option<moves::Technique> {
        self.move_four.map(|x| x.0)
    }
    // Setter methods
    //
    /// Sets the non volatile status initial
    pub fn set_non_volatile(&mut self, status: enums::NonVolatile) {
        self.non_volatile_status = (status, 0);
    }

    /// Sets the moves to the pokemon
    pub fn set_moves(&mut self, moves: Vec<moves::Technique>) {
        if moves.len() > 1 {
            self.move_one = Some((moves[0].clone(), moves[0].get_power_points().unwrap()));
        }
        if moves.len() > 1 {
            self.move_two = Some((moves[1].clone(), moves[1].get_power_points().unwrap()));
        }
        if moves.len() > 2 {
            self.move_three = Some((moves[2].clone(), moves[2].get_power_points().unwrap()));
        }
        if moves.len() > 3 {
            self.move_four = Some((moves[3].clone(), moves[3].get_power_points().unwrap()));
        }
    }

    /// Sets the type of the pokemon
    pub fn set_type(&mut self, position: u8, change: enums::Types) {
        match position {
            0 => self.type_one = change,
            1 => self.type_two = change,
            _ => unreachable!(),
        }
    }
    // Other methods
    //
    /// Adds an end of turn flag
    pub fn add_end_flag(&mut self, flag: enums::EndOfTurn) {
        self.end_of_turn_flags.insert(flag, 0);
    }
    pub fn add_choose_flag(&mut self, flag: enums::Choose) {
        self.choose_flags.insert(flag, 0);
    }
    pub fn add_resolve_flag(&mut self, flag: enums::Resolve) {
        self.resolve_flags.insert(flag, 0);
    }
    pub fn add_fight_flag(&mut self, flag: enums::Fighting) {
        self.fight_flags.insert(flag, 0);
    }
    /// Checks if the pokemon is asleep
    pub fn is_asleep(&self) -> bool {
        self.non_volatile_status.0 == enums::NonVolatile::Sleep
    }
    /// Checks if the pokemon is alive
    pub fn is_alive(&mut self) -> bool {
        self.get_current().get_stat(&enums::Stats::Hp) > 0
    }
    /// Increments the Non Volatile Counter
    pub fn increment_non_volatile(&mut self) {
        self.non_volatile_status.1 += 1;
    }
    /// Decrements the AP
    pub fn decrement_ap(&mut self) {
        unimplemented!();
    }
}
