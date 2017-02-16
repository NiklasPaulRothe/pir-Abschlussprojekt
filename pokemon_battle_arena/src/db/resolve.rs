extern crate rand;
extern crate regex;

use super::enums;
use super::moves::Technique;
use super::pokemon_token::PokemonToken;
use self::rand::{Rng, thread_rng};
use self::regex::Regex;
use arena::Arena;
use graphic;
use player::Player;


/// Resolves moves that simply deals damage to the opponent, or the damage part of more complex
/// moves
pub fn deal_damage(attack: &Technique,
                   user: &mut PokemonToken,
                   target: &mut PokemonToken,
                   player: &mut Player,
                   mut window: &mut graphic::gui::App)
                   -> u16 {
    let mut stab = 1.0;
    let mut rng = thread_rng();
    let random = rng.gen_range(0.85, 1.0);
    if attack.get_type() == user.get_types().0 || attack.get_type() == user.get_types().1 {
        stab = 1.5;
    }
    // First check for special or physical attack
    let attack_stat: enums::Stats;
    let defense_stat: enums::Stats;
    if attack.get_damage_class() == enums::DamageClass::Physical {
        attack_stat = enums::Stats::Attack;
        defense_stat = enums::Stats::Defense;
    } else {
        attack_stat = enums::Stats::SpecialAttack;
        defense_stat = enums::Stats::SpecialDefense;
    }
    // get power for the move, calculate if it has a variable value
    let power: u16;
    if attack.get_power().is_none() {
        power = get_power(attack, user, target, &mut window);
    } else {
        power = attack.get_power().unwrap();
    }
    if power == 0 {
        return 0;
    }
    let modifier = stab * attack.get_effectiveness(target.clone(), &mut window) * random;
    let mut damage = ((((2.0 * user.get_level() as f32 + 10.0) / 250.0) *
                       user.get_current().get_stat(&attack_stat) as f32 /
                       target.get_current().get_stat(&defense_stat) as f32 *
                       power as f32 + 2.0) * modifier) as u16;

    if attack.get_damage_class() == enums::DamageClass::Physical &&
       player.get_flags().contains_key(&enums::PlayerFlag::Reflect) {
        damage = damage / 2;
    } else if attack.get_damage_class() == enums::DamageClass::Special &&
              player.get_flags().contains_key(&enums::PlayerFlag::LightScreen) {
        damage = damage / 2;
    }
    let critical = match attack.get_crit_rate() {
        0 => rng.gen_range(0.0, 100.1) <= 6.25,
        1 => rng.gen_range(0.0, 100.1) <= 12.5,
        2 => rng.gen_range(0.0, 100.1) <= 50.0,
        _ => true,
    };
    if critical {
        window.set_battle_text("Critical Hit".to_string());
        damage = (damage as f32 * 1.5) as u16;
    }
    let current = target.get_current().get_stat(&enums::Stats::Hp);
    target.get_current().set_stats(enums::Stats::Hp, current - damage);
    damage
}

// Resolves ailment effects
pub fn ailment(name: &str,
               move_type: enums::Types,
               ailment: enums::Ailment,
               effect_chance: u8,
               user: PokemonToken,
               target: &mut PokemonToken,
               player: &mut Player,
               window: &mut graphic::gui::App) {
    let mut rng = thread_rng();
    let random = rng.gen_range(0, 101);
    // Only works if the effect chance of the move is met.
    let probability = effect_chance;
    if random <= probability {
        let powder = Regex::new(r"powder").unwrap();
        let spore = Regex::new(r"spore").unwrap();
        let tmp: &str = &name;
        // some sort of attacks did not work against grass types.
        if (target.get_types().0 == enums::Types::Grass ||
            target.get_types().1 == enums::Types::Grass) &&
           (powder.is_match(tmp) || spore.is_match(tmp)) {
            window.set_battle_text(target.get_name() + " was not affected by " + name);
        } else {
            // Categorize the moves by the ailment they cause. Ailments usually automatically fail
            // if the target already was hit by a move that caused the same ailment and still
            // suffer from it's effect. Non volatile Ailments even fail if the target is under the
            // effect of one of these kind.
            match ailment {

                enums::Ailment::Paralysis => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // electric type pokemon are immune to paralysis
                        if target.get_non_volatile().0 == enums::NonVolatile::Undefined {
                            if !(target.get_types().0 == enums::Types::Electric) &&
                               !(target.get_types().1 == enums::Types::Electric) {
                                target.set_non_volatile(enums::NonVolatile::Paralysis);
                                let base = target.get_base().clone();
                                target.get_current().set_stats(enums::Stats::Speed,
                                                               base.get_stat(&enums::Stats::Speed) /
                                                               2);
                                window.set_battle_text(target.get_name() + " was paralysed.");
                            } else {
                                window.set_battle_text(target.get_name() + " was not affected by " +
                                                     name);
                            }
                        } else {
                            window.set_battle_text(target.get_name() +" is already " +
                                &enums::print_non_volatile(target.get_non_volatile().0));
                        }
                    } else {
                        window.set_battle_text(target.get_name() + " was protected.");
                    }
                }

                enums::Ailment::Sleep => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        if target.get_non_volatile().0 == enums::NonVolatile::Undefined {
                            window.set_battle_text(target.get_name() + " slept in.");
                            target.set_non_volatile(enums::NonVolatile::Sleep)
                        } else {
                            window.set_battle_text(target.get_name() +" is already " +
                                &enums::print_non_volatile(target.get_non_volatile().0));
                        }
                    } else {
                        window.set_battle_text(target.get_name() + " was protected.");
                    }
                }

                enums::Ailment::Freeze => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // ice type pokemon are immune to freeze, but only if the used move is also
                        // from the type ice.
                        if (target.get_types().0 == enums::Types::Ice ||
                            target.get_types().1 == enums::Types::Ice) &&
                           move_type == enums::Types::Ice {
                            window.set_battle_text(target.get_name() + " could not be freezed.");
                        } else {
                            target.set_non_volatile(enums::NonVolatile::Freeze);
                            window.set_battle_text(target.get_name() + " was freezed.");
                        }
                    } else {
                        window.set_battle_text(target.get_name() + " was protected.");
                    }
                }

                enums::Ailment::Burn => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // Fire types can not be burned (seems logical).
                        if target.get_types().0 == enums::Types::Fire ||
                           target.get_types().1 == enums::Types::Fire {
                            window.set_battle_text(target.get_name() + " could not be burned.");
                        } else {
                            target.set_non_volatile(enums::NonVolatile::Burn);
                            window.set_battle_text(target.get_name() + " was burned.");
                        }
                    } else {
                        window.set_battle_text(target.get_name() + " was protected.");
                    }
                }

                enums::Ailment::Poison => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // Neither Poison nor steel pokemon can be poisoned in normal ways.
                        if target.get_types().0 == enums::Types::Poison ||
                           target.get_types().0 == enums::Types::Steel ||
                           target.get_types().1 == enums::Types::Poison ||
                           target.get_types().1 == enums::Types::Steel {
                            window.set_battle_text(target.get_name() + " could not be poisoned.");
                        } else {
                            if name == "toxic" {
                                target.set_non_volatile(enums::NonVolatile::BadPoison);
                                window.set_battle_text(target.get_name() + " was badly poisoned.");
                            } else {
                                target.set_non_volatile(enums::NonVolatile::Poison);
                                window.set_battle_text(target.get_name() + " was poisoned.");
                            }
                        }
                    } else {
                        window.set_battle_text(target.get_name() + " was protected.");
                    }
                }

                enums::Ailment::LeechSeed => {
                    // Has no effect on grass type (even though given the flavor text leech seeds
                    // are a plant parasite...)
                    if target.get_types().0 == enums::Types::Grass ||
                       target.get_types().1 == enums::Types::Grass {
                        window.set_battle_text(target.get_name() +
                                               " was not affected by Leech Seed.");
                    } else {
                        target.add_end_flag(enums::EndOfTurn::LeechSeed);
                    }
                }

                enums::Ailment::PerishSong => {
                    // actually only one Attack, that kills all Pokemon after 4 rounds, including
                    // the user. Does not reset the counter if used again, therefore Pokemon, that
                    // are already under the effect of Perish Song are not influenced
                    if target.get_end_of_turn_flags().contains_key(&enums::EndOfTurn::PerishSong) {
                        window.set_battle_text(target.get_name() + " is already doomed.");
                    } else {
                        target.add_end_flag(enums::EndOfTurn::PerishSong);
                    }
                }

                enums::Ailment::Yawn => {
                    if target.get_end_of_turn_flags().contains_key(&enums::EndOfTurn::Yawn) ||
                       target.get_non_volatile().0 == enums::NonVolatile::Sleep {
                        window.set_battle_text(target.get_name() + " was not affected by Yawn.");
                    } else {
                        target.add_end_flag(enums::EndOfTurn::Yawn);
                    }
                }

                enums::Ailment::Trap => {
                    if !target.get_end_of_turn_flags().contains_key(&enums::EndOfTurn::Trap) {
                        target.add_end_flag(enums::EndOfTurn::Trap);
                    }
                }

                enums::Ailment::Confusion => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        if !target.get_fight_flags().contains_key(&enums::Fighting::Confusion) {
                            target.add_fight_flag(enums::Fighting::Confusion);
                        }
                    } else {
                        window.set_battle_text(target.get_name() + " was protected.");
                    }
                }

                enums::Ailment::NoTypeImmunity => {
                    if !target.get_resolve_flags().contains_key(&enums::Resolve::NoTypeImmunity) {
                        target.add_resolve_flag(enums::Resolve::NoTypeImmunity)
                    }
                }

                enums::Ailment::HealBlock => {
                    if !target.get_resolve_flags().contains_key(&enums::Resolve::HealBlock) {
                        target.add_resolve_flag(enums::Resolve::HealBlock)
                    }
                }

                enums::Ailment::Ingrain => {
                    if !target.get_end_of_turn_flags().contains_key(&enums::EndOfTurn::Ingrain) {
                        target.add_end_flag(enums::EndOfTurn::Ingrain);
                        if target.get_fight_flags().contains_key(&enums::Fighting::Infatuation) {
                            target.get_fight_flags().remove(&enums::Fighting::Infatuation);
                        }
                    }
                }

                // Unimplemented for now. Would prevent the Player from using Items on a Pokemon,
                // but as far as there are no items it has no effect.
                enums::Ailment::Embargo => {}

                enums::Ailment::Torment => {
                    if !target.get_choose_flags().contains_key(&enums::Choose::Torment) {
                        target.add_choose_flag(enums::Choose::Torment)
                    }
                }

                enums::Ailment::Infatuation => {
                    if !target.get_fight_flags().contains_key(&enums::Fighting::Infatuation) &&
                       target.get_gender() != user.get_gender() {
                        target.add_fight_flag(enums::Fighting::Infatuation);
                    } else {
                        window.set_battle_text(target.get_name() + " was not affected by attract.");
                    }
                }

                enums::Ailment::Unknown => {
                    if !target.get_resolve_flags().contains_key(&enums::Resolve::Telekinesis) {
                        target.add_resolve_flag(enums::Resolve::Telekinesis);
                    }
                }

                _ => {}
            }
        }
    }
}

/// Calculates the new value of a stat given a specific change of stages.
pub fn change_stats(stages: i8,
                    stat: enums::Stats,
                    target: &mut PokemonToken,
                    defender: &mut Player,
                    window: &mut graphic::gui::App)
                    -> bool {
    if defender.get_flags().contains_key(&enums::PlayerFlag::Mist) {
        return false;
    }
    // calculate current stage and cap stages at -6 and 6
    let stage = get_stages(stat, target);
    println!("{:?}", stage);
    if !(stage <= -6 && stage >= 6) {
        let mut new_stage = stage + stages;
        if new_stage > 6 {
            new_stage = 6;
        } else if new_stage < -6 {
            new_stage = -6
        }
        // if target is paralysed the current speed value is set to half of it's value.
        let mut modifier = 1.0;
        if target.get_non_volatile().0 == enums::NonVolatile::Paralysis &&
           stat == enums::Stats::Speed {
            modifier = 0.5;
        }
        let base = target.get_base().get_stat(&stat) as f32;

        let new_stat = modifier *
                       match stat {
            enums::Stats::Accuracy => {
                match new_stage {
                    -6 => base / 3.0,
                    -5 => base / 8.0 * 3.0,
                    -4 => base / 7.0 * 3.0,
                    -3 => base / 2.0,
                    -2 => base / 5.0 * 3.0,
                    -1 => base / 4.0 * 3.0,
                    0 => base,
                    1 => base / 3.0 * 4.0,
                    2 => base / 3.0 * 5.0,
                    3 => base * 2.0,
                    4 => base / 3.0 * 7.0,
                    5 => base / 3.0 * 8.0,
                    6 => base * 3.0,
                    _ => base,
                }
            }
            enums::Stats::Evasion => {
                match new_stage {
                    6 => base / 3.0,
                    5 => base / 8.0 * 3.0,
                    4 => base / 7.0 * 3.0,
                    3 => base / 2.0,
                    2 => base / 5.0 * 3.0,
                    1 => base / 4.0 * 3.0,
                    0 => base,
                    -1 => base / 3.0 * 4.0,
                    -2 => base / 3.0 * 5.0,
                    -3 => base * 2.0,
                    -4 => base / 3.0 * 7.0,
                    -5 => base / 3.0 * 8.0,
                    -6 => base * 3.0,
                    _ => base,
                }
            }
            _ => {
                match new_stage {
                    -6 => base / 4.0,
                    -5 => base / 7.0 * 2.0,
                    -4 => base / 3.0,
                    -3 => base / 5.0 * 2.0,
                    -2 => base / 2.0,
                    -1 => base / 3.0 * 2.0,
                    0 => base,
                    1 => base * 1.5,
                    2 => base * 2.0,
                    3 => base * 2.5,
                    4 => base * 3.0,
                    5 => base * 3.5,
                    6 => base * 4.0,
                    _ => base,
                }
            }
        };
        target.get_current().set_stats(stat, new_stat as u16);
        return true;
    } else {
        if stage < -6 {
            window.set_battle_text(target.get_name() + "s " + enums::stat_to_string(stat) +
                                   " can not be lowered anymore");
        }
    }
    return false;
}

fn get_stages(stat: enums::Stats, target: &mut PokemonToken) -> i8 {
    let mut current = target.get_current().get_stat(&stat);
    if target.get_non_volatile().0 == enums::NonVolatile::Paralysis && stat == enums::Stats::Speed {
        current = current * 2;
    }
    match stat {
        enums::Stats::Accuracy => {
            match (current as f32 / target.get_base().get_stat(&stat) as f32) * 100.0 {
                0.0...34.0 => -6,
                34.0...38.0 => -5,
                38.0...43.0 => -4,
                43.0...51.0 => -3,
                51.0...61.0 => -2,
                61.0...76.0 => -1,
                76.0...101.0 => 0,
                101.0...134.0 => 1,
                134.0...166.0 => 2,
                166.0...201.0 => 3,
                201.0...234.0 => 4,
                234.0...267.0 => 5,
                267.0...301.0 => 6,
                _ => 0,
            }
        }
        enums::Stats::Evasion => {
            match (current as f32 / target.get_base().get_stat(&stat) as f32) * 100.0 {
                0.0...34.0 => 6,
                34.0...38.0 => 5,
                38.0...43.0 => 4,
                43.0...51.0 => 3,
                51.0...61.0 => 2,
                61.0...76.0 => 1,
                76.0...101.0 => 0,
                101.0...134.0 => -1,
                134.0...166.0 => -2,
                166.0...201.0 => -3,
                201.0...234.0 => -4,
                234.0...267.0 => -5,
                267.0...301.0 => -6,
                _ => 0,
            }
        }
        _ => {
            match (current as f32 / target.get_base().get_stat(&stat) as f32) * 100.0 {
                0.0...26.0 => -6,
                26.0...29.0 => -5,
                29.0...34.0 => -4,
                34.0...41.0 => -3,
                41.0...50.0 => -2,
                50.0...67.0 => -1,
                67.0...101.0 => 0,
                101.0...151.0 => 1,
                151.0...201.0 => 2,
                201.0...251.0 => 3,
                251.0...301.0 => 4,
                301.0...351.0 => 5,
                351.0...401.0 => 6,
                _ => 0,
            }
        }
    }
}


// Heals the targets HP by the provided value, or, if this would raise the HP above the base stat,
// to their base HP.
pub fn heal(target: &mut PokemonToken, value: u16, window: &mut graphic::gui::App) {
    if !target.get_resolve_flags().contains_key(&enums::Resolve::HealBlock) {
        if value + target.get_current().get_stat(&enums::Stats::Hp) >=
           target.get_base().get_stat(&enums::Stats::Hp) {
            let base = target.get_base().clone();
            target.get_current().set_stats(enums::Stats::Hp, base.get_stat(&enums::Stats::Hp));
        } else {
            let current = target.get_current().clone();
            target.get_current().set_stats(enums::Stats::Hp,
                                           (current.get_stat(&enums::Stats::Hp) + value));
        }
    } else {
        window.set_battle_text(target.get_name() + " could not be healed.");
    }
}

// Switches the Pokemon of the target Player
pub fn switch_pokemon(target: &mut Player) {
    let alive = target.get_alive_count();
    if alive > 1 {
        let mut rng = thread_rng();
        let range = rng.gen_range(0, alive - 1);
        let id = target.clone().get_alive_list()[range];
        let mut position = 0;
        for elem in target.clone().get_pokemon_list() {
            if elem.get_id() == id {
                target.set_current(position);
                break;
            }
            position += 1;
        }
    }
}

// Simply sets the HP of the target to 0 (Thats what K.O. means I suppose.)
pub fn ko_attack(target: &mut PokemonToken) {
    target.get_current().set_stats(enums::Stats::Hp, 0);
}

/// Resolves the attack "haze". Changes all Stats to default.
pub fn haze(pokemon: &mut PokemonToken) {
    let clone = pokemon.get_base().clone();
    pokemon.get_current().set_stats(enums::Stats::Attack, clone.get_stat(&enums::Stats::Attack));
    pokemon.get_current().set_stats(enums::Stats::Defense,
                                    clone.get_stat(&enums::Stats::Defense));
    pokemon.get_current().set_stats(enums::Stats::SpecialAttack,
                                    clone.get_stat(&enums::Stats::SpecialAttack));
    pokemon.get_current().set_stats(enums::Stats::SpecialDefense,
                                    clone.get_stat(&enums::Stats::SpecialDefense));
    pokemon.get_current().set_stats(enums::Stats::Speed, clone.get_stat(&enums::Stats::Speed));
    pokemon.get_current().set_stats(enums::Stats::Accuracy,
                                    clone.get_stat(&enums::Stats::Accuracy));
    pokemon.get_current().set_stats(enums::Stats::Evasion,
                                    clone.get_stat(&enums::Stats::Evasion));
}

/// Resolves the field effects. Returns false if a new effect is set. True if it was not allowed
pub fn field_effects(arena: &mut Arena, effect: enums::FieldEffects) -> bool {
    if !arena.get_field_effects().contains_key(&effect) {
        arena.get_field_effects().insert(effect, 0);
        return false;
    }
    true
}
/// Resolves the terrain WholeFieldEffects
pub fn terrain(arena: &mut Arena, effect: enums::FieldEffects) -> bool {

    match effect {
        enums::FieldEffects::GrassyTerrain => {
            if arena.get_field_effects().contains_key(&enums::FieldEffects::MistyTerrain) {
                arena.get_field_effects().remove(&enums::FieldEffects::MistyTerrain);
            } else if arena.get_field_effects()
                .contains_key(&enums::FieldEffects::ElectricTerrain) {
                arena.get_field_effects().remove(&enums::FieldEffects::ElectricTerrain);
            }
        }
        enums::FieldEffects::MistyTerrain => {
            if arena.get_field_effects().contains_key(&enums::FieldEffects::GrassyTerrain) {
                arena.get_field_effects().remove(&enums::FieldEffects::GrassyTerrain);
            } else if arena.get_field_effects()
                .contains_key(&enums::FieldEffects::ElectricTerrain) {
                arena.get_field_effects().remove(&enums::FieldEffects::ElectricTerrain);
            }
        }
        enums::FieldEffects::ElectricTerrain => {
            if arena.get_field_effects().contains_key(&enums::FieldEffects::MistyTerrain) {
                arena.get_field_effects().remove(&enums::FieldEffects::MistyTerrain);
            } else if arena.get_field_effects().contains_key(&enums::FieldEffects::GrassyTerrain) {
                arena.get_field_effects().remove(&enums::FieldEffects::GrassyTerrain);
            }
        }
        _ => unreachable!(),

    }
    field_effects(arena, effect)
}
/// Resolves the special field effect trick room, magic room and wonder room
pub fn rooms(arena: &mut Arena, effect: enums::FieldEffects) -> bool {
    if field_effects(arena, effect) {
        arena.get_field_effects().remove(&effect);
    }
    false
}
/// Resolves the weather. Returns false if no error accured and a new weather could be set
pub fn weather(arena: &mut Arena, weather: enums::Weather) -> bool {
    if arena.get_current_weather().0 != weather {
        arena.set_current_weather(weather);
        return false;
    }
    true
}

/// Simply adds flags for one side of the field. They must be resolved in the fight method of the
/// arena.
pub fn field_effect(attack: &Technique, player: &mut Player) {
    match attack.get_name() {
        "sticky-web" => player.add_flag(enums::PlayerFlag::StickyWeb),
        "stealth-rock" => player.add_flag(enums::PlayerFlag::StealthRock),
        "toxic-spikes" => player.add_flag(enums::PlayerFlag::ToxicSpikes),
        "lucky-chant" => player.add_flag(enums::PlayerFlag::LuckyChant),
        "spikes" => player.add_flag(enums::PlayerFlag::Spikes),
        "crafty-shield" => player.add_flag(enums::PlayerFlag::CraftyShield),
        "mat-block" => player.add_flag(enums::PlayerFlag::MatBlock),
        "quick-guard" => player.add_flag(enums::PlayerFlag::QuickGuard),
        "wide-guard" => player.add_flag(enums::PlayerFlag::WideGuard),
        "tailwind" => player.add_flag(enums::PlayerFlag::Tailwind),
        "safeguard" => player.add_flag(enums::PlayerFlag::Safeguard),
        "reflect" => player.add_flag(enums::PlayerFlag::Reflect),
        "light-screen" => player.add_flag(enums::PlayerFlag::LightScreen),
        "mist" => player.add_flag(enums::PlayerFlag::Mist),
        _ => {}
    }

}

/// Returns the power for a move with variable value
pub fn get_power(attack: &Technique,
                 user: &mut PokemonToken,
                 target: &mut PokemonToken,
                 mut window: &mut graphic::gui::App)
                 -> u16 {
    let mut rng = thread_rng();
    match attack.get_name() {
        "sonic-boom" => {
            if target.get_types().1 != enums::Types::Ghost &&
               target.get_types().0 != enums::Types::Ghost {
                let current = target.get_current().get_stat(&enums::Stats::Hp);
                target.get_current().set_stats(enums::Stats::Hp, current - 20);
            } else {
                window.set_battle_text("It has no effect on ".to_string() + &target.get_name());
            }
            0
        }
        "low-kick" => {
            match user.get_weight() {
                0...10 => 20,
                10...25 => 40,
                25...50 => 60,
                50...100 => 80,
                100...200 => 100,
                _ => 120,
            }
        }
        "counter" => {
            // need a way to get the last attack that hits the pokemon and if it was in this round
            0
        }
        "seismic-toss" => {
            if target.get_types().1 != enums::Types::Ghost &&
               target.get_types().0 != enums::Types::Ghost {
                let current = target.get_current().get_stat(&enums::Stats::Hp);
                target.get_current().set_stats(enums::Stats::Hp, current - user.get_level());
            } else {
                window.set_battle_text("It has no effect on ".to_string() + &target.get_name());
            }
            0
        }
        "dragon-rage" => {
            let current = target.get_current().get_stat(&enums::Stats::Hp);
            target.get_current().set_stats(enums::Stats::Hp, current - 40);
            0
        }
        "night-shade" => {
            if target.get_types().1 != enums::Types::Normal &&
               target.get_types().0 != enums::Types::Normal {
                let current = target.get_current().get_stat(&enums::Stats::Hp);
                target.get_current().set_stats(enums::Stats::Hp, current - user.get_level());
            } else {
                window.set_battle_text("It has no effect on ".to_string() + &target.get_name());
            }
            0
        }
        "bide" => {
            // see counter
            0
        }
        "psywave" => {
            let current = target.get_current().get_stat(&enums::Stats::Hp);
            target.get_current()
                .set_stats(enums::Stats::Hp,
                           current - (user.get_level() as f32 * rng.gen_range(0.5, 1.5)) as u16);
            0
        }
        "super-fang" => {
            if target.get_types().1 != enums::Types::Ghost &&
               target.get_types().0 != enums::Types::Ghost {
                let current = target.get_current().get_stat(&enums::Stats::Hp);
                target.get_current().set_stats(enums::Stats::Hp, current - current / 2);
            } else {
                window.set_battle_text("It has no effect on ".to_string() + &target.get_name());
            }
            0
        }
        "flail" => {
            match target.get_current().get_stat(&enums::Stats::Hp) as f32 /
                  target.get_base().get_stat(&enums::Stats::Hp) as f32 {
                0.0...4.17 => 200,
                4.17...10.42 => 150,
                10.42...20.83 => 100,
                20.83...35.42 => 80,
                35.42...68.75 => 40,
                _ => 20,
            }
        }
        "reversal" => {
            match target.get_current().get_stat(&enums::Stats::Hp) as f32 /
                  target.get_base().get_stat(&enums::Stats::Hp) as f32 {
                0.0...4.17 => 200,
                4.17...10.42 => 150,
                10.42...20.83 => 100,
                20.83...35.42 => 80,
                35.42...68.75 => 40,
                _ => 20,
            }
        }
        "return" => 50,
        "present" => {
            match rng.gen_range(0, 101) {
                0...41 => 40,
                41...71 => 80,
                71...81 => 120,
                _ => {
                    let value = target.get_base().get_stat(&enums::Stats::Hp) / 4;
                    heal(target, value, &mut window);
                    0
                }
            }
        }
        "frustration" => 50,
        "magnitude" => {
            match rng.gen_range(0, 101) {
                0...6 => 10,
                6...16 => 30,
                16...36 => 50,
                63...66 => 70,
                66...86 => 90,
                86...96 => 110,
                _ => 150,
            }
        }
        "mirror-coat" => {
            // see counter
            0
        }
        "beat-up" => user.get_current().get_stat(&enums::Stats::Attack) / 10 + 5,
        "spit-up" => {
            // can be changed as soon as stockpile is completely implemented
            0
        }
        "endeavor" => {
            target.get_current().set_stats(enums::Stats::Hp,
                                           user.get_current().get_stat(&enums::Stats::Hp));
            0
        }
        "gyro-ball" => {
            25 *
            (target.get_current().get_stat(&enums::Stats::Speed) /
             user.get_current().get_stat(&enums::Stats::Speed))
        }
        "natural-gift" => {
            // Right now it is not possible to hold a berry
            0
        }
        "metal-burst" => {
            // see counter
            0
        }
        "fling" => {
            // Right now it is not possible to hold a item
            0
        }
        "trump-card" => {
            // PP not implemented right now, therefore we just use an average value for now
            60
        }
        "wring-out" => {
            1 +
            120 *
            (target.get_current().get_stat(&enums::Stats::Hp) /
             target.get_base().get_stat(&enums::Stats::Hp))
        }
        "me-first" => {
            // see counter
            0
        }
        "punishment" => {
            let mut stages = Vec::new();
            stages.push(get_stages(enums::Stats::Attack, target));
            stages.push(get_stages(enums::Stats::Defense, target));
            stages.push(get_stages(enums::Stats::SpecialAttack, target));
            stages.push(get_stages(enums::Stats::SpecialDefense, target));
            stages.push(get_stages(enums::Stats::Speed, target));
            stages.push(get_stages(enums::Stats::Accuracy, target));
            stages.push(get_stages(enums::Stats::Evasion, target));
            let mut increase = 0;
            for entry in stages {
                if entry > 0 {
                    increase = increase + entry;
                }
            }
            60 + (20 * increase) as u16
        }
        "grass-knot" => {
            match target.get_weight() {
                0...10 => 20,
                10...25 => 40,
                25...50 => 60,
                50...100 => 80,
                100...200 => 100,
                _ => 120,
            }
        }
        "crush-grip" => {
            1 +
            120 *
            (target.get_current().get_stat(&enums::Stats::Hp) /
             target.get_base().get_stat(&enums::Stats::Hp))
        }
        "heavy-slam" => {
            match user.get_weight() as f32 / target.get_weight() as f32 * 100.0 {
                0.0...20.1 => 120,
                20.1...25.1 => 100,
                25.1...33.4 => 80,
                33.4...50.1 => 60,
                _ => 40,
            }
        }
        "electro-ball" => {
            match user.get_current().get_stat(&enums::Stats::Speed) as f32 /
                  target.get_current().get_stat(&enums::Stats::Speed) as f32 {
                0.0...25.1 => 150,
                25.1...33.4 => 120,
                33.4...50.1 => 80,
                _ => 60,
            }
        }
        "final-gambit" => {
            let user_current = user.get_current().get_stat(&enums::Stats::Hp);
            let current = target.get_current().get_stat(&enums::Stats::Hp);
            target.get_current().set_stats(enums::Stats::Hp, current - user_current);
            user.get_current().set_stats(enums::Stats::Hp, 0);
            0
        }
        "heat-crash" => {
            match user.get_weight() as f32 / target.get_weight() as f32 * 100.0 {
                0.0...20.1 => 120,
                20.1...25.1 => 100,
                25.1...33.4 => 80,
                33.4...50.1 => 60,
                _ => 40,
            }
        }
        _ => 0,

    }
}
