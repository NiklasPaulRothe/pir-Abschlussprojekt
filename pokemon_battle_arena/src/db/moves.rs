extern crate csv;
extern crate num;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;

use super::enums;
use super::pokemon_token;
use super::resolve;
use self::num::FromPrimitive;
use self::rand::{Rng, thread_rng};
use self::regex::Regex;
use arena::Arena;
use graphic;
use player::{Player, Next};
use std::cmp::Ordering;
use std::collections::HashMap;
use super::unique;


/// Struct that is a representation of a move a pokemon can learn. Contains everything that is
/// needed to calculate it's impact given a user and a target Pokemon.
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
    /// Matches over the category of a move and calls a specific method in resolve.rs for this
    /// category. All calculation is done inside the method, therefore no return is needed.
    pub fn resolve(&self,
                   mut arena: &mut Arena,
                   flag: enums::Player,
                   mut window: &mut graphic::gui::App) {
        // First call the hits method to sort out missing moves.
        let mut user_clone = get_user(flag, arena).clone();
        let mut target_clone = get_target(flag, arena).clone();
        let mut defender_clone = get_defender(flag, arena).clone();
        let mut attacker_clone = get_attacker(flag, arena).clone();
        if self.hits(&mut target_clone, &mut user_clone, &mut defender_clone) {
            // Match over the category provides smaller samples that must be dealt with.
            match self.get_category() {

                enums::MoveCategory::Damage => {
                    // Set only the last action if move needs to be charged or recharged
                    if self.get_flags().contains(&enums::MoveFlags::Charge) &&
                       attacker_clone.get_last_action().clone() != (Next::Move(self.clone()), 0) {
                        let attacker = get_attacker(flag, arena);
                        window.set_battle_text(user_clone.get_name().to_string() +
                                               " prepared itself");
                        attacker.set_last_action(((Next::Move(self.clone()), 0)));
                        attacker.set_next_move(Some(Next::Move(self.clone())));
                        return;
                    } else if self.get_flags().contains(&enums::MoveFlags::Recharge) &&
                              attacker_clone.get_last_action().clone() ==
                              (Next::Move(self.clone()), 0) {
                        let attacker = get_attacker(flag, arena);
                        window.set_battle_text(user_clone.get_name() + " has to recharge");
                        attacker.set_last_action((Next::Move(self.clone()), 1));
                        return;
                    }
                    let mut rng = thread_rng();
                    // scope to destroy target afterwards, otherwise flinch could not be resolved
                    {
                        let mut target = get_target(flag, arena);
                        let mut frequency = 1;
                        // multiple hits of one attack will be resolved one by one
                        if self.min_hits.is_some() {
                            frequency =
                                rng.gen_range(self.min_hits.unwrap(), self.max_hits.unwrap());
                        }
                        let name: &str = &target.get_name();
                        window.set_battle_text(user_clone.get_name() + " hits " + name);
                        if frequency > 1 {
                            window.set_battle_text(self.get_name().to_string() + " hits " +
                                                   &frequency.to_string() +
                                                   " times");
                        }
                        for _ in 0..frequency {
                            resolve::deal_damage(&self,
                                                 &mut user_clone,
                                                 &mut target,
                                                 &mut defender_clone,
                                                 window);
                        }
                    }
                    // resolve flinch chance if available
                    if self.flinch_chance > 0 &&
                       rng.gen_range(0.0, 100.1) <= self.flinch_chance as f32 {
                        get_defender(flag, arena).set_next_move(Some(Next::Flinch));
                        window.set_battle_text(target_clone.get_name() + " flinched.");
                    }
                    if self.get_flags().contains(&enums::MoveFlags::Recharge) &&
                       attacker_clone.get_last_action().clone() != (Next::Move(self.clone()), 0) {
                        let attacker = get_attacker(flag, arena);
                        attacker.set_next_move(Some(Next::Move(self.clone())));
                    }
                }

                enums::MoveCategory::Ailment => {
                    let mut target = get_target(flag, arena);
                    resolve::ailment(self.get_name(),
                                     self.get_type(),
                                     self.get_ailment(),
                                     100,
                                     user_clone,
                                     &mut target,
                                     &mut defender_clone,
                                     &mut window);
                }

                enums::MoveCategory::NetGoodStats => {
                    let mut target = get_user(flag, arena);
                    // in long effect the stats that should be in- or decreased are given, this
                    // makes it possible to get matches with a regex. Several if statements are
                    // needed, because it is possible to influence more than one stat.
                    if Regex::new(r"attack").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::Attack,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                    if Regex::new(r"defense").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::Defense,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                    if Regex::new(r"special-attack").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::SpecialAttack,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                    if Regex::new(r"special-defense").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::SpecialDefense,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                    if Regex::new(r"speed").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::Speed,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                    if Regex::new(r"accuracy").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::Accuracy,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                    if Regex::new(r"evasion").unwrap().is_match(&self.effect_long) {
                        resolve::change_stats(self.stat_change_rate.unwrap(),
                                              enums::Stats::Evasion,
                                              &mut target,
                                              &mut attacker_clone,
                                              &mut window);
                    }
                }

                enums::MoveCategory::Heal => {
                    let weather = arena.get_current_weather().clone();
                    let mut user = get_user(flag, arena);
                    // Heal moves will fail if the user has maximum HP
                    if !(user.get_current().get_stat(&enums::Stats::Hp) ==
                         user.get_base().get_stat(&enums::Stats::Hp)) {
                        let value: u16;
                        // Deal with moves that heal different amounts of HP for different
                        // weather conditions.
                        if (self.get_name() == String::from("moonlight")) ||
                           (self.get_name() == String::from("synthesis")) ||
                           (self.get_name() == String::from("morning-sun")) {
                            match weather.0 {
                                enums::Weather::ClearSky => {
                                    value = user.get_base().get_stat(&enums::Stats::Hp) / 2;
                                }
                                enums::Weather::Sunlight => {
                                    value = (user.get_base().get_stat(&enums::Stats::Hp) / 4) * 3;
                                }
                                _ => {
                                    if self.get_name() == String::from("morning-sun") {
                                        value = user.get_base().get_stat(&enums::Stats::Hp) / 4
                                    } else {
                                        value = user.get_base().get_stat(&enums::Stats::Hp) / 8
                                    }
                                }
                            };
                            resolve::heal(&mut user, value, &mut window);
                        } else if self.get_name() == String::from("heal-pulse") {
                            resolve::heal(&mut user, 50, &mut window);
                            // TShe use of swallow is bound to a former use of stockpile
                        } else if self.get_name() == String::from("swallow") {
                            // TODO: find a way to get a percentage according to the use of
                            // stockpile in the rounds before
                            resolve::heal(&mut user, 25, &mut window);
                            // Besides healing roost changes the type of pokemon with type
                            // flying.
                        } else if self.get_name() == String::from("roost") {
                            // TODO: find a way to change type of user for one round
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
                            resolve::heal(&mut user, 50, &mut window);
                        } else {
                            resolve::heal(&mut user, 50, &mut window);
                        }

                        window.set_battle_text(user_clone.get_name() + " heals " +
                                               &target_clone.get_name())
                    } else {
                        println!("{} failed", self.get_name());
                    }
                }

                enums::MoveCategory::DamageAndAilment => {
                    let mut target = get_target(flag, arena);
                    window.set_battle_text(user_clone.get_name() + " hits " + &target.get_name());
                    resolve::deal_damage(&self,
                                         &mut user_clone,
                                         &mut target,
                                         &mut defender_clone,
                                         window);
                    resolve::ailment(self.get_name(),
                                     self.get_type(),
                                     self.get_ailment(),
                                     self.get_effect_chance(),
                                     user_clone,
                                     &mut target,
                                     &mut defender_clone,
                                     &mut window);
                }

                // Swagger moves confuse the target and raise their stats. Important is that the
                // stats will be raised even when the target is already confused or can not be
                // confused due to other reasons, but it will not get confused if the stats can
                // not be raised anymore.
                enums::MoveCategory::Swagger => {
                    let mut defender = get_defender(flag, arena).clone();
                    let mut target = get_target(flag, arena);
                    if resolve::change_stats(self.get_stat_change_rate(),
                                             self.get_stat(),
                                             &mut target,
                                             &mut defender_clone,
                                             &mut window) {
                        resolve::ailment(self.get_name(),
                                         self.get_type(),
                                         self.get_ailment(),
                                         100,
                                         user_clone,
                                         &mut target,
                                         &mut defender,
                                         &mut window);
                    }
                }

                enums::MoveCategory::DamageAndLower => {
                    let mut target = get_target(flag, arena);
                    window.set_battle_text(user_clone.get_name() + " hits " +
                                           &target_clone.get_name());
                    resolve::deal_damage(&self,
                                         &mut user_clone,
                                         &mut target,
                                         &mut defender_clone,
                                         window);
                    resolve::change_stats(self.get_stat_change_rate(),
                                          self.get_stat(),
                                          &mut target,
                                          &mut defender_clone,
                                          &mut window);
                }

                enums::MoveCategory::DamageAndRaise => {
                    let mut target = get_target(flag, arena);
                    window.set_battle_text(user_clone.get_name() + " hits " +
                                           &target_clone.get_name());
                    resolve::deal_damage(&self,
                                         &mut user_clone,
                                         &mut target,
                                         &mut defender_clone,
                                         window);
                    resolve::change_stats(self.get_stat_change_rate(),
                                          self.get_stat(),
                                          &mut target,
                                          &mut attacker_clone,
                                          &mut window);
                }

                // First deals damage and afterwards heals themselve for a percentage of the
                // dealt damage.
                enums::MoveCategory::DamageAndHeal => {
                    // dream eater can only be used if the target is asleep
                    if self.get_name() == "dream-eater" && !target_clone.is_asleep() {
                        println!("Dream Eater failed");
                    } else {
                        let mut value: u16;
                        {
                            let mut target = get_target(flag, arena);
                            window.set_battle_text(user_clone.get_name() + " hits " +
                                                   &target_clone.get_name());
                            value = resolve::deal_damage(&self,
                                                         &mut user_clone,
                                                         &mut target,
                                                         &mut defender_clone,
                                                         window);
                            match self.get_drain_percentage() {
                                50 => value = value / 2,
                                75 => value = (value / 4) * 3,
                                _ => unreachable!(),
                            }
                        }
                        let mut user = get_user(flag, arena);
                        resolve::heal(&mut user, value, &mut window);
                        window.set_battle_text(user.get_name() + " absorbed HP");
                    }
                }

                // K.O. Attacks that instantly let the target faint if hitting. Besides low
                // accuracy every K.O. Attack has another requirement, that must be met for it
                // to work.
                enums::MoveCategory::Ohko => {
                    let mut target = get_target(flag, arena);
                    if ((self.get_name() == String::from("guillotine") ||
                         self.get_name() == String::from("sheer-cold")) &&
                        user_clone.get_level() >= target.get_level()) ||
                       ((self.get_name() == String::from("horn-drill") ||
                         self.get_name() == String::from("fissure")) &&
                        user_clone.get_current().get_stat(&enums::Stats::Speed) >=
                        target.get_current().get_stat(&enums::Stats::Speed)) {
                        resolve::ko_attack(&mut target);
                    } else {
                        println!("{} was not affected by {}",
                                 target.get_name(),
                                 self.get_name());
                    }
                }

                enums::MoveCategory::WholeFieldEffect => {
                    let mut failure = true;
                    if self.get_name() == String::from("haze") {
                        {
                            let mut target = get_target(flag, arena);
                            resolve::haze(target);
                        }
                        {
                            let mut user = get_user(flag, arena);
                            resolve::haze(user);
                        }
                        failure = false;
                    } else if self.get_name() == String::from("sandstorm") {
                        failure = resolve::weather(arena, enums::Weather::Sandstorm);
                    } else if self.get_name() == String::from("rain-dance") {
                        failure = resolve::weather(arena, enums::Weather::Rain);
                    } else if self.get_name() == String::from("sunny-day") {
                        failure = resolve::weather(arena, enums::Weather::Sunlight);
                    } else if self.get_name() == String::from("hail") {
                        failure = resolve::weather(arena, enums::Weather::Hail);
                    } else if self.get_name() == String::from("mud-sport") {
                        failure = resolve::field_effects(arena, enums::FieldEffects::MudSport);
                    } else if self.get_name() == String::from("water-sport") {
                        failure = resolve::field_effects(arena, enums::FieldEffects::WaterSport);
                    } else if self.get_name() == String::from("gravity") {
                        failure = resolve::field_effects(arena, enums::FieldEffects::Gravity);
                    } else if self.get_name() == String::from("ion-deluge") {
                        failure = resolve::field_effects(arena, enums::FieldEffects::IonDeluge);
                    } else if self.get_name() == String::from("fairy-lock") {
                        failure = resolve::field_effects(arena, enums::FieldEffects::FairyLock);
                    } else if self.get_name() == String::from("grassy-terrain") {
                        failure = resolve::terrain(arena, enums::FieldEffects::GrassyTerrain);
                    } else if self.get_name() == String::from("misty-terrain") {
                        failure = resolve::terrain(arena, enums::FieldEffects::MistyTerrain);
                    } else if self.get_name() == String::from("electric-terrain") {
                        failure = resolve::terrain(arena, enums::FieldEffects::ElectricTerrain);
                    } else if self.get_name() == String::from("trick-room") {
                        failure = resolve::rooms(arena, enums::FieldEffects::TrickRoom);
                    } else if self.get_name() == String::from("wonder-room") {
                        failure = resolve::rooms(arena, enums::FieldEffects::WonderRoom);
                    } else if self.get_name() == String::from("magic-room") {
                        failure = resolve::rooms(arena, enums::FieldEffects::MagicRoom);
                    }

                    if failure {
                        println!("Failed to resolve {}.", self.get_name());
                    }
                }

                enums::MoveCategory::FieldEffect => {
                    let target_side = Regex::new(r"opposing").unwrap();
                    if target_side.is_match(&self.effect_long) {
                        resolve::field_effect(self, get_defender(flag, arena));
                    } else {
                        resolve::field_effect(self, get_attacker(flag, arena));
                    }
                }

                enums::MoveCategory::ForceSwitch => {
                    let mut defender = &mut arena.get_player_two();
                    if target_clone.get_level() <= user_clone.get_level() {
                        resolve::switch_pokemon(&mut defender);
                    } else {
                        println!("It has no effect on {}", target_clone.get_name());
                    }
                }
                enums::MoveCategory::Unique => {
                    unique::unique(&self.clone(),
                                   self.get_name(),
                                   user_clone,
                                   target_clone,
                                   &mut attacker_clone,
                                   &mut defender_clone,
                                   &mut arena,
                                   flag,
                                   &mut window);
                }
            };
        } else {
            window.set_battle_text(user_clone.get_name() + " misses " + &target_clone.get_name());
        }
        // sets the last action to the action that was really executed in the last turn and the last
        // move, which is the last actions that counts in terms of moves like mimic
        {
            let attacker = get_attacker(flag, arena);
            if self.get_flags().contains(&enums::MoveFlags::Charge) &&
               attacker.get_last_action().0 == Next::Move(self.clone()) {
                attacker.set_last_action((Next::Move(self.clone()), 1))
            } else if self.get_min_turn() > 1 &&
                      attacker.get_last_action().1 < self.get_max_turns() &&
                      attacker.get_last_action().0 == Next::Move(self.clone()) {
                let turns = attacker.get_last_action().1 + 1;
                attacker.set_last_action((Next::Move(self.clone()), turns));
            } else {
                attacker.set_last_action((Next::Move(self.clone()), 0));
                attacker.set_last_move(Some(self.clone()));

            }
        }
        // handles a case in which the HP of a target are set to a value less than 0
        let target = get_target(flag, arena);
        if target.get_current().get_stat(&enums::Stats::Hp) > 2000 {
            target.get_current().set_stats(enums::Stats::Hp, 0);
        }
    }

    /// Checks if the attacking pokemon is hitting the enemy. Returns true if the target will be
    /// hit by the user and false if not
    pub fn hits(&self,
                user: &mut pokemon_token::PokemonToken,
                target: &mut pokemon_token::PokemonToken,
                player: &mut Player)
                -> bool {
        // TODO: As soon as flags for semi invulnerability are added, they have to be taken mut
        // account for hit calculation.

        // first resolve special cases in which certain moves will always fail or hit
        if self.get_flags().contains(&enums::MoveFlags::Protect) &&
           target.get_resolve_flags().contains_key(&enums::Resolve::Protect) {
            return false;
        }
        if self.get_name() == "mat-block" && !player.get_switched() {
            return false;
        }
        if player.get_flags().contains_key(&enums::PlayerFlag::MatBlock) &&
           !self.get_flags().contains(&enums::MoveFlags::Protect) {
            return false;
        }
        if player.get_flags().contains_key(&enums::PlayerFlag::WideGuard) &&
           (self.get_target() == enums::Target::AllOtherPokemon ||
            self.get_target() == enums::Target::AllOpponents ||
            self.get_target() == enums::Target::UserAndAllies ||
            self.get_target() == enums::Target::AllPokemon) {
            return false;
        }
        if target.get_resolve_flags().contains_key(&enums::Resolve::Telekinesis) &&
           self.get_category() == enums::MoveCategory::Ohko {
            return true;
        }
        let probability: u16;
        if self.accuracy.is_some() {
            if player.get_flags().contains_key(&enums::PlayerFlag::CraftyShield) &&
               self.get_name() != "perish-song" &&
               self.get_damage_class() == enums::DamageClass::Status {
                return false;
            }
            let mut modifier = user.get_current().get_stat(&enums::Stats::Accuracy) /
                               target.get_current().get_stat(&enums::Stats::Evasion);
            if modifier < 33 {
                modifier = 33;
            }
            probability = self.accuracy.unwrap() * modifier;
        } else {
            return true;
        }
        let mut rng = thread_rng();
        let random = rng.gen_range(0, 101);
        if random <= probability {
            return true;
        }
        false
    }

    // Getter Methods
    //
    /// Takes the attacked Pokemon as an input besides the move and calculate from their types
    /// how effective the move is. Returns an appropriate enum for further calculations.
    pub fn get_effectiveness(&self,
                             mut enemy: pokemon_token::PokemonToken,
                             window: &mut graphic::gui::App)
                             -> f32 {
        let mut eff_count = 0;
        if self.clone().effectivity_map.unwrap().contains_key(&enemy.get_types().0) {
            if !((enemy.get_types().0 == enums::Types::Ghost &&
                  *self.clone().effectivity_map.unwrap().get(&enums::Types::Ghost).unwrap() ==
                  -4) &&
                 enemy.get_resolve_flags().contains_key(&enums::Resolve::NoTypeImmunity)) {
                eff_count = eff_count +
                            self.clone()
                    .effectivity_map
                    .unwrap()
                    .get(&enemy.get_types().0)
                    .unwrap();
            }
        }
        if enemy.get_types().1 != enums::Types::Undefined &&
           self.clone().effectivity_map.unwrap().contains_key(&enemy.get_types().1) {
            if !((enemy.get_types().1 == enums::Types::Ghost &&
                  *self.clone().effectivity_map.unwrap().get(&enums::Types::Ghost).unwrap() ==
                  -4) &&
                 enemy.get_resolve_flags().contains_key(&enums::Resolve::NoTypeImmunity)) {
                eff_count = eff_count +
                            self.clone()
                    .effectivity_map
                    .unwrap()
                    .get(&enemy.get_types().1)
                    .unwrap();
            }
        }
        match eff_count {
            -2 => 0.25,
            -1 => 0.5,
            0 => 1.0,
            1 => 2.0,
            2 => 4.0,
            _ => {
                window.set_battle_text(enemy.get_name() + " is immune");
                0.0
            }
        }
    }
    /// Gets the id of the attack
    pub fn get_id(&self) -> usize {
        self.attack_id
    }
    /// Gets the name of the attack
    pub fn get_name(&self) -> &str {
        &self.name
    }
    // Takes a Vec<Technique> and returns a Vec<String> with the names of the techniques
    pub fn get_name_vec(technique: Vec<Technique>) -> Vec<String> {
        let mut output = Vec::new();

        for entry in technique {
            output.push(String::from(entry.get_name()));
        }

        output
    }
    /// Gets the type of the attack
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
    /// Gets the power of the attack. None if no damage can be applied
    pub fn get_power(&self) -> Option<u16> {
        self.power
    }
    /// Gets the ap of the attack
    pub fn get_power_points(&self) -> Option<u8> {
        self.power_points
    }
    /// Gets the accuracy of the attack
    pub fn get_accuracy(&self) -> Option<u16> {
        self.accuracy
    }
    /// Gets the priority of the attack.
    pub fn get_priority(&self) -> i8 {
        self.priority
    }

    /// Gets the possible targets which will be hit by using this attack
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
    /// Gets the damage class of the attack
    pub fn get_damage_class(&self) -> enums::DamageClass {
        let tmp: &str = &self.damage_class;
        match tmp {
            "physical" => enums::DamageClass::Physical,
            "special" => enums::DamageClass::Special,
            "status" => enums::DamageClass::Status,
            _ => unreachable!(),
        }
    }
    /// Gets a short description of the attack effect
    pub fn get_short_effect(&self) -> String {
        self.effect_short.clone()
    }
    /// Gets a long description of the attack effect
    pub fn get_long_effect(&self) -> String {
        self.effect_long.clone()
    }
    /// Gets the chance an effect will hit
    pub fn get_effect_chance(&self) -> u8 {
        if self.effect_chance.is_some() {
            return self.effect_chance.unwrap();
        }
        100
    }
    /// Gets the category of the attack. (e.g. heal, damage...)
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
    /// Gets the possible ailment caused by the attack
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
    /// Gets the min amount of hits the attack can do (e.g. Double Kick with two)
    pub fn get_min_hits(&self) -> u8 {
        if self.min_hits.is_some() {
            return self.min_hits.unwrap();
        }
        1
    }
    /// Gets the max amount of hits the attack can do (e.g. Double Kick with two)
    pub fn get_max_hits(&self) -> u8 {
        if self.max_hits.is_some() {
            return self.max_hits.unwrap();
        }
        1
    }
    /// Gets the mininum duration of the attack used in rounds
    pub fn get_min_turn(&self) -> u8 {
        if self.min_turns.is_some() {
            return self.min_turns.unwrap();
        }
        1
    }
    /// Gets the maximum duration of the attack used in rounds
    pub fn get_max_turns(&self) -> u8 {
        if self.max_turns.is_some() {
            return self.max_turns.unwrap();
        }
        1
    }
    /// Gets the drain percentage
    pub fn get_drain_percentage(&self) -> i8 {
        self.drain_percentage
    }
    /// Gets the healing percentage
    pub fn get_healing_percentage(&self) -> i8 {
        self.healing_percentage
    }
    /// Gets the chance for a critical hit
    pub fn get_crit_rate(&self) -> u8 {
        self.crit_rate
    }
    /// Gets the chance the additional ailment will hit
    pub fn get_ailment_chance(&self) -> u8 {
        self.ailment_chance
    }
    /// Gets the chance the enemy pokemon will flinch
    pub fn get_flinch_chance(&self) -> u8 {
        self.flinch_chance
    }
    /// Gets the chance that the attack causes stat changes
    pub fn get_stat_chance(&self) -> u8 {
        self.stat_chance
    }
    /// Gets the description of attack readable by the user of the program
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    /// Gets the stat that will be changed by the attack
    pub fn get_stat(&self) -> enums::Stats {
        if self.stat.is_some() {
            return enums::Stats::from_i32(self.stat.unwrap()).unwrap();
        }
        enums::Stats::from_i32(0).unwrap()
    }
    /// Gets the multiplier stage a stat will be changed with
    pub fn get_stat_change_rate(&self) -> i8 {
        if self.stat_change_rate.is_some() {
            return self.stat_change_rate.unwrap();
        }
        0
    }
    /// Gets the effectivity map
    pub fn get_effectivity_map(&self) -> HashMap<enums::Types, i8> {
        self.clone().effectivity_map.unwrap()
    }
    /// Gets the style of the attack
    pub fn get_flags(&self) -> Vec<enums::MoveFlags> {
        if self.move_flags.is_some() {
            return self.move_flags.clone().unwrap();
        }
        Vec::new()
    }
    /// Sets the effectivity map
    pub fn set_effectivity_map(&mut self, map: HashMap<enums::Types, i8>) {
        self.effectivity_map = Some(map);
    }
    /// Sets the move flags
    pub fn set_flags(&mut self, flag: Vec<enums::MoveFlags>) {
        self.move_flags = Some(flag);
    }
}

impl Ord for Technique {
    fn cmp(&self, other: &Technique) -> Ordering {
        self.attack_id.cmp((&other.attack_id))
    }
}

impl PartialOrd for Technique {
    fn partial_cmp(&self, other: &Technique) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Technique {
    fn eq(&self, other: &Technique) -> bool {
        self.attack_id == other.attack_id
    }
}

impl Eq for Technique {}

/// Helper function which will get the mutable reference of the targets pokemon out of an arena
pub fn get_target<'a>(target: enums::Player,
                      arena: &'a mut Arena)
                      -> &'a mut pokemon_token::PokemonToken {
    match target {
        enums::Player::One => {
            let current = arena.get_player_one().get_current();
            &mut arena.get_player_one().get_pokemon_list()[current]
        }
        enums::Player::Two => {
            let current = arena.get_player_two().get_current();
            &mut arena.get_player_two().get_pokemon_list()[current]
        }
    }
}

/// Helper function which will get the mutable reference of the users pokemon out of an arena
pub fn get_user<'a>(target: enums::Player,
                    arena: &'a mut Arena)
                    -> &'a mut pokemon_token::PokemonToken {
    match target {
        enums::Player::Two => {
            let current = arena.get_player_one().get_current();
            &mut arena.get_player_one().get_pokemon_list()[current]
        }
        enums::Player::One => {
            let current = arena.get_player_two().get_current();
            &mut arena.get_player_two().get_pokemon_list()[current]
        }
    }
}

pub fn get_attacker<'a>(target: enums::Player, arena: &'a mut Arena) -> &'a mut Player {
    match target {
        enums::Player::One => arena.get_player_one(),
        enums::Player::Two => arena.get_player_two(),
    }
}

pub fn get_defender<'a>(target: enums::Player, arena: &'a mut Arena) -> &'a mut Player {
    match target {
        enums::Player::One => arena.get_player_two(),
        enums::Player::Two => arena.get_player_one(),
    }
}
