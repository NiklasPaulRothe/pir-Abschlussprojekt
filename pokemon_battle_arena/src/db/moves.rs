extern crate csv;
extern crate num;
extern crate rustc_serialize;
extern crate rand;

use super::pokemon_token;
use super::enums;
use super::resolve;
use self::num::FromPrimitive;
use self::rand::{Rng, thread_rng};
use std::collections::HashMap;
use player::Player;
use arena::Arena;

///Struct that is a representation of a move a pokemon can learn. Contains everything that is
///needed to calculate it's impact given a user and a target Pokemon.
#[derive(Debug, Clone, RustcDecodable)]
pub struct Technique {
    attack_id: usize,
    name: String,
    attack_type: String,
    power: Option<u16>,
    power_points: Option<u8>,
    accuracy: Option<u16>,
    priority: i8,
    target: String,
    damage_class: String,
    effect_short: String,
    effect_long: String,
    effect_chance: Option<u8>,
    category: String,
    ailment: String,
    min_hits: Option<u8>,
    max_hits: Option<u8>,
    min_turns: Option<u8>,
    max_turns: Option<u8>,
    drain_percentage: i8,
    healing_percentage: i8,
    crit_rate: u8,
    ailment_chance: u8,
    flinch_chance: u8,
    stat_chance: u8,
    description: String,
    stat: Option<i32>,
    effectivity_map: Option<HashMap<enums::Types, i8>>,
    move_flags: Option<Vec<enums::MoveFlags>>,
    stat_change_rate: Option<i8>,
}

impl Technique {
    ///Matches over the category of a move and calls a specific method in resolve.rs for this
    ///category. All calculation is done inside the method, therefore no return is needed.
    pub fn resolve(&self, mut user: pokemon_token::PokemonToken,
        mut targets: Vec<pokemon_token::PokemonToken>, attacker: Player, defender: Player, field: Arena) {
        //if no target is provided push user as target, so that the loop afterwards works, also
        //attacks that have no target usually affect the field or the user.
        if targets.is_empty() {
            targets.push(user.clone());
        }

        //for loop that calls the resolve methods for every choosen target.
        for target in targets.clone() {
            //first call the hits method to sort out missing moves.
            if self.hits(&target, &user) {
                //match over the category provides smaller samples that must be dealt with.
                match self.get_category() {

                    enums::MoveCategory::Damage => {
                        resolve::deal_damage(self.clone(), user.clone(), target);
                    },

                    enums::MoveCategory::Ailment => {
                        resolve::ailment(self.get_name(), self.get_type(), self.get_ailment(), 100, target);
                    },

                    enums::MoveCategory::NetGoodStats => {},

                    enums::MoveCategory::Heal => {
                        //Heal moves will fail if the user has maximum HP
                        if !(user.get_current().get_stat(enums::Stats::Hp) ==
                            user.get_base().get_stat(enums::Stats::Hp)) {
                            let value: u16;
                            //Deal with moves that heal different amounts of HP for different
                            //weather conditions.
                            if (self.get_name() == String::from("moonlight")) ||
                            (self.get_name() == String::from("synthesis")) ||
                            (self.get_name() == String::from("morning-sun")) {
                                match field.get_weather() {
                                    enums::Weather::ClearSky => {
                                        value = user.get_base().get_stat(enums::Stats::Hp) / 2;
                                    },
                                    enums::Weather::Sunlight => {
                                        value = (user.get_base().get_stat(enums::Stats::Hp)
                                         / 4) * 3;
                                    },
                                    _ => {
                                        if self.get_name() == String::from("morning-sun") {
                                            value = user.get_base().
                                                get_stat(enums::Stats::Hp) / 4
                                        } else {
                                            value = user.get_base().
                                                get_stat(enums::Stats::Hp) / 8
                                        }
                                    }
                                };
                                resolve::heal(target, value);
                            } else if self.get_name() == String::from("heal-pulse") {
                                resolve::heal(target, 50);
                                //the use of swallow is bound to a former use of stockpile
                            } else if self.get_name() == String::from("swallow") {
                                //TODO: find a way to get a percentage according to the use of
                                //stockpile in the rounds before
                                resolve::heal(target, 25);
                            //besides healing roost changes the type of pokemon with type
                            //flying.
                            } else if self.get_name() == String::from("roost") {
                                //TODO: find a way to change type of user for one round
                                if user.get_types().1 != enums::Types::Undefined &&
                                user.get_types().0 == enums::Types::Flying {
                                    user.set_type(0, enums::Types::Undefined);
                                    user.add_end_flag(enums::EndOfTurn::RoostTypeOne);
                                } else if user.get_types().1 == enums::Types::Flying {
                                    user.set_type(1, enums::Types::Undefined);
                                    user.add_end_flag(enums::EndOfTurn::RoostTypeTwo);
                                } else if user.get_types().0 == enums::Types::Flying {
                                    user.set_type(0, enums::Types::Normal);
                                    user.add_end_flag(enums::EndOfTurn::RoostTypeOne)
                                }
                                resolve::heal(target, 50);
                            } else {
                                resolve::heal(user.clone(), 50);
                            }


                        } else {
                            println!("{} failed", self.get_name());
                        }
                    },

                    enums::MoveCategory::DamageAndAilment => {
                        resolve::deal_damage(self.clone(), user.clone(), target.clone());
                        resolve::ailment(self.get_name(), self.get_type(), self.get_ailment(),
                            self.get_effect_chance(), target);
                    },

                    //apart from the Math done
                    //Swagger moves confuse the target and raise their stats. Important is that the
                    //stats will be raised even when the target is already confused or can not be
                    //confused due to other reasons, but it will not get confused if the stats can
                    //not be raised anymore.
                    enums::MoveCategory::Swagger => {
                        if resolve::change_stats(self.get_stat_change_rate(), self.get_stat(),
                            target.clone()) {
                            resolve::ailment(self.get_name(), self.get_type(), self.get_ailment(),
                             100, target);
                        }
                    },

                    enums::MoveCategory::DamageAndLower => {
                        resolve::deal_damage(self.clone(), user.clone(), target.clone());
                        resolve::change_stats(self.get_stat_change_rate(), self.get_stat(), target);
                    },

                    enums::MoveCategory::DamageAndRaise => {
                        resolve::deal_damage(self.clone(), user.clone(), target.clone());
                        resolve::change_stats(self.get_stat_change_rate(), self.get_stat(), target);
                    },

                    //done apart from math for damage
                    //First deals damage and afterwards heals themselve for a percentage of the
                    //dealt damage.
                    enums::MoveCategory::DamageAndHeal => {
                        //dream eater can only be used if the target is asleep
                        if self.get_name() == String::from("dream-eater")
                        && !target.is_asleep() {
                            println!("Dream Eater failed");
                        } else {
                            let mut value = resolve::deal_damage(self.clone(), user.clone(),
                                target.clone());
                            match self.get_drain_percentage() {
                                50 => value = value / 2,
                                75 => value = (value / 4) * 3,
                                _ => unreachable!(),
                            }
                            resolve::heal(user.clone(), value);
                        }
                    },

                    //totally done
                    //K.O. Attacks that instantly let the target faint if hitting. Besides low
                    //accuracy every K.O. Attack has another requirement, that must be met for it
                    //to work.
                    enums::MoveCategory::Ohko => {
                        if ((self.get_name() == String::from("guillotine") ||
                            self.get_name() == String::from("sheer-cold")) &&
                        user.get_level() >= target.get_level()) ||
                        ((self.get_name() == String::from("horn-drill") ||
                        self.get_name() == String::from("fissure")) &&
                        user.get_current().get_stat(enums::Stats::Speed)
                        >= target.get_current().get_stat(enums::Stats::Speed))  {
                            resolve::ko_attack(target);
                        } else {
                            println!("{} was not affected by {}", target.get_name(),
                                self.get_name());
                        }
                    },

                    enums::MoveCategory::WholeFieldEffect => {},

                    enums::MoveCategory::FieldEffect => {},

                    enums::MoveCategory::ForceSwitch => {
                        if target.get_level() <= user.get_level() {
                            resolve::switch_pokemon(defender.clone());
                        } else {
                            println!("It has no effect on {}", target.get_name());
                        }
                    },

                    enums::MoveCategory::Unique => {},
                };
            } else {
                println!("{} missed {}", user.get_name(), target.get_name());
            }
        }
    }

    pub fn hits(&self, user: &pokemon_token::PokemonToken, target: &pokemon_token::PokemonToken)
        -> bool {
            //TODO: As soon as flags for semi invulnerability are added, they have to be taken into
            //account for hit calculation.
        let mut probability = 100;
        if self.accuracy.is_some() {
            probability = self.accuracy.unwrap() * (user.get_current().get_stat(enums::Stats::Accuracy) /
                target.get_current().get_stat(enums::Stats::Evasion))
        }
        let mut rng = thread_rng();
        let random = rng.gen_range(0, 101);
        if random <= probability {
            return true;
        }
        false
    }

    ///Takes the attacked Pokemon as an input besides the move and calculate from their types
    ///how effective the move is. Returns an appropriate enum for further calculations.
    pub fn get_effectiveness(&self, enemy: pokemon_token::PokemonToken) -> f32 {
        let mut eff_count = 0;
        if self.clone().effectivity_map.unwrap().contains_key(&enemy.get_types().0) {
            eff_count = eff_count + self.clone().effectivity_map.unwrap().get(&enemy.get_types().0)
            .unwrap();
        }
        if enemy.get_types().1 != enums::Types::Undefined
        && self.clone().effectivity_map.unwrap().contains_key(&enemy.get_types().1) {
            eff_count = eff_count + self.clone().effectivity_map.unwrap().get(&enemy.get_types().1)
            .unwrap();
        }
        match eff_count {
            -2 => 0.25,
            -1 => 0.5,
            0 => 1.0,
            1 => 2.0,
            2 => 4.0,
            _ => 0.0,
        }
    }

    pub fn get_id(&self) -> usize {
        self.attack_id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    // Takes a Vec<Technique> and returns a Vec<String> with the names of the techniques
    pub fn get_name_vec(technique: Vec<Technique>) -> Vec<String> {
        let mut output = Vec::new();

        for entry in technique {
            output.push(entry.get_name());
        }

        output
    }

    pub fn get_type(&self) -> enums::Types {
        let a_type: &str = &self.attack_type;
        match a_type {
            "normal" => enums::Types::Normal,
            "fighting" => enums::Types::Fighting,
            "flying" => enums::Types::Flying,
            "poison" => enums::Types::Poison,
            "ground" => enums::Types::Ground,
            "rock" => enums::Types::Rock,
            "bug" => enums::Types::Bug,
            "ghost" => enums::Types::Ghost,
            "steel" => enums::Types::Steel,
            "fire" => enums::Types::Fire,
            "water" => enums::Types::Water,
            "grass" => enums::Types::Grass,
            "electric" => enums::Types::Electric,
            "psychic" => enums::Types::Psychic,
            "ice" => enums::Types::Ice,
            "dragon" => enums::Types::Dragon,
            "dark" => enums::Types::Dark,
            "fairy" => enums::Types::Fairy,
            _ => enums::Types::Undefined,
        }
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

    pub fn get_priority(&self) -> i8 {
        self.priority
    }

    pub fn get_target(&self) -> enums::Target {
        let tmp: &str = &self.target;
        match tmp {
            "specific-move" => enums::Target::SpecificMove,
            "selected-pokemon-me-first" => enums::Target::SelectedPokemonMeFirst,
            "ally" => enums::Target::Ally,
            "users-field" => enums::Target::UsersField,
            "user-or-ally" => enums::Target::UserOrAlly,
            "opponents-field" => enums::Target::OpponentsField,
            "user" => enums::Target::User,
            "random-opponent" => enums::Target::RandomOpponent,
            "all-other-pokemon" => enums::Target::AllOtherPokemon,
            "selected-pokemon" => enums::Target::SelectedPokemon,
            "all-opponents" => enums::Target::AllOpponents,
            "entire-field" => enums::Target::EntireField,
            "user-and-allies" => enums::Target::UserAndAllies,
            "all-pokemon" => enums::Target::AllPokemon,
            _ => unreachable!(),
        }
    }

    pub fn get_damage_class(&self) -> enums::DamageClass {
        let tmp: &str = &self.damage_class;
        match tmp {
            "physical" => enums::DamageClass::Physical,
            "special" => enums::DamageClass::Special,
            "status" => enums::DamageClass::Status,
            _ => unreachable!(),
        }
    }

    pub fn get_short_effect(&self) -> String {
        self.effect_short.clone()
    }

    pub fn get_long_effect(&self) -> String {
        self.effect_long.clone()
    }

    pub fn get_effect_chance(&self) -> u8 {
        if self.effect_chance.is_some() {
            return self.effect_chance.unwrap();
        }
        100
    }

    pub fn get_category(&self) -> enums::MoveCategory {
        let tmp: &str = &self.category;
        match tmp {
            "damage" => enums::MoveCategory::Damage,
            "ailment" => enums::MoveCategory::Ailment,
            "net-good-stats" => enums::MoveCategory::NetGoodStats,
            "heal" => enums::MoveCategory::Heal,
            "damage+ailment" => enums::MoveCategory::DamageAndAilment,
            "swagger" => enums::MoveCategory::Swagger,
            "damage+lower" => enums::MoveCategory::DamageAndLower,
            "damage+raise" => enums::MoveCategory::DamageAndRaise,
            "damage+heal" => enums::MoveCategory::DamageAndHeal,
            "ohko" => enums::MoveCategory::Ohko,
            "whole-field-effect" => enums::MoveCategory::WholeFieldEffect,
            "field-effect" => enums::MoveCategory::FieldEffect,
            "force-switch" => enums::MoveCategory::ForceSwitch,
            "unique" => enums::MoveCategory::Unique,
            _ => unreachable!(),
        }
    }

    pub fn get_ailment(&self) -> enums::Ailment {
        let tmp: &str = &self.ailment;
        match tmp {
            "unknown" => enums::Ailment::Unknown,
            "none" => enums::Ailment::Undefined,
            "paralysis" => enums::Ailment::Paralysis,
            "sleep" => enums::Ailment::Sleep,
            "freeze" => enums::Ailment::Freeze,
            "burn" => enums::Ailment::Burn,
            "poison" => enums::Ailment::Poison,
            "confusion" => enums::Ailment::Confusion,
            "infatuation" => enums::Ailment::Infatuation,
            "trap" => enums::Ailment::Trap,
            "nightmare" => enums::Ailment::Nightmare,
            "torment" => enums::Ailment::Torment,
            "disable" => enums::Ailment::Disable,
            "yawn" => enums::Ailment::Yawn,
            "heal-block" => enums::Ailment::HealBlock,
            "no-type-immunity" => enums::Ailment::NoTypeImmunity,
            "leech-seed" => enums::Ailment::LeechSeed,
            "embargo" => enums::Ailment::Embargo,
            "perish-song" => enums::Ailment::PerishSong,
            "ingrain" => enums::Ailment::Ingrain,
            _ => unreachable!(),
        }
    }

    pub fn get_min_hits(&self) -> u8 {
        if self.min_hits.is_some() {
            return self.min_hits.unwrap();
        }
        1
    }

    pub fn get_max_hits(&self) -> u8 {
        if self.max_hits.is_some() {
            return self.max_hits.unwrap();
        }
        1
    }

    pub fn get_min_turn(&self) -> u8 {
        if self.min_turns.is_some() {
            return self.min_turns.unwrap();
        }
        1
    }

    pub fn get_max_turns(&self) -> u8 {
        if self.max_turns.is_some() {
            return self.max_turns.unwrap();
        }
        1
    }

    pub fn get_drain_percentage(&self) -> i8 {
        self.drain_percentage
    }

    pub fn get_healing_percentage(&self) -> i8 {
        self.healing_percentage
    }

    pub fn get_crit_rate(&self) -> u8 {
        self.crit_rate
    }

    pub fn get_ailment_chance(&self) -> u8 {
        self.ailment_chance
    }

    pub fn get_flinch_chance(&self) -> u8 {
        self.flinch_chance
    }

    pub fn get_stat_chance(&self) -> u8 {
        self.stat_chance
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_stat(&self) -> enums::Stats {
        if self.stat.is_some(){
            return enums::Stats::from_i32(self.stat.unwrap()).unwrap();
        }
        enums::Stats::from_i32(0).unwrap()
    }

    pub fn get_stat_change_rate(&self) -> i8 {
        if self.stat_change_rate.is_some() {
            return self.stat_change_rate.unwrap();
        }
        0
    }

    pub fn get_effectivity_map(&self) -> HashMap<enums::Types, i8> {
        self.clone().effectivity_map.unwrap()
    }

    pub fn get_flags(&self) -> Vec<enums::MoveFlags> {
        if self.move_flags.is_some() {
            return self.move_flags.clone().unwrap();
        }
        Vec::new()
    }

    pub fn set_effectivity_map(&mut self, map: HashMap<enums::Types, i8>) {
        self.effectivity_map = Some(map);
    }

    pub fn set_flags(&mut self, flag: Vec<enums::MoveFlags>) {
        self.move_flags = Some(flag);
    }
}
