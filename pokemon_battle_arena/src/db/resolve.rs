extern crate rand;
extern crate regex;

use super::moves::Technique;
use super::pokemon_token::PokemonToken;
use super::enums;
use arena::Arena;
use self::rand::{Rng, thread_rng};
use self::regex::Regex;
use player::Player;

/// Resolves moves that simply deals damage to the opponent.
pub fn deal_damage(attack: &Technique,
                   user: &mut PokemonToken,
                   target: &mut PokemonToken,
                   player: &mut Player)
                   -> u16 {
    // TODO: Methode die matcht zwischen Attacken die direkt verrechnet werden können und denen,
    // die variable Power haben. Hier muss eine Möglichkeit gefunden werden die Power möglichst
    // effizient für alle Attacken zu berechnen.
    let mut stab = 1.0;
    let mut rng = thread_rng();
    let random = rng.gen_range(0.85, 1.0);
    if attack.get_type() == user.get_types().0 || attack.get_type() == user.get_types().1 {
        stab = 1.5;
    }
    let attack_stat: enums::Stats;
    let defense_stat: enums::Stats;
    if attack.get_damage_class() == enums::DamageClass::Physical {
        attack_stat = enums::Stats::Attack;
        defense_stat = enums::Stats::Defense;
    } else {
        attack_stat = enums::Stats::SpecialAttack;
        defense_stat = enums::Stats::SpecialDefense;
    }
    let mut damage = 0;
    if attack.get_power().is_some() {
        let modifier = stab * attack.get_effectiveness(target.clone()) * random;
        damage = ((((2.0 * user.get_level() as f32 + 10.0) / 250.0) *
                   user.get_current().get_stat(&attack_stat) as f32 /
                   target.get_current().get_stat(&defense_stat) as f32 *
                   attack.get_power().unwrap() as f32 + 2.0) * modifier) as u16;
    }
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
        damage = (damage as f32 * 1.5) as u16;
    }
    let current = target.get_current().get_stat(&enums::Stats::Hp);
    target.get_current().set_stats(enums::Stats::Hp, current - damage);
    println!("Damage: {}", damage);
    println!("HP in resolve: {}",
             target.get_current().get_stat(&enums::Stats::Hp));
    damage
}

// Resolves ailment effects
pub fn ailment(name: &str,
               move_type: enums::Types,
               ailment: enums::Ailment,
               effect_chance: u8,
               user: PokemonToken,
               target: &mut PokemonToken,
               player: &mut Player) {
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
            println!("{} was not affected by {}", target.get_name(), name);
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
                                                               2)
                            } else {
                                println!("{} was not affected by {}", target.get_name(), name);
                            }
                        } else {
                            println!("{} is already {}",
                                     target.get_name(),
                                     enums::print_non_volatile(target.get_non_volatile().0));
                        }
                    } else {
                        println!("{} was protected", target.get_name());
                    }
                }

                enums::Ailment::Sleep => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        if target.get_non_volatile().0 == enums::NonVolatile::Undefined {
                            target.set_non_volatile(enums::NonVolatile::Sleep)
                        } else {
                            println!("{} is already {}",
                                     target.get_name(),
                                     enums::print_non_volatile(target.get_non_volatile().0));
                        }
                    } else {
                        println!("{} was protected", target.get_name());
                    }
                }

                enums::Ailment::Freeze => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // ice type pokemon are immune to freeze, but only if the used move is also
                        // from the type ice.
                        if (target.get_types().0 == enums::Types::Ice ||
                            target.get_types().1 == enums::Types::Ice) &&
                           move_type == enums::Types::Ice {
                            println!("{} could not be freezed", target.get_name());
                        } else {
                            target.set_non_volatile(enums::NonVolatile::Freeze);
                        }
                    } else {
                        println!("{} was protected", target.get_name());
                    }
                }

                enums::Ailment::Burn => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // Fire types can not be burned (seems logical).
                        if target.get_types().0 == enums::Types::Fire ||
                           target.get_types().1 == enums::Types::Fire {
                            println!("{} could not be burned", target.get_name());
                        } else {
                            target.set_non_volatile(enums::NonVolatile::Burn);
                        }
                    } else {
                        println!("{} was protected", target.get_name());
                    }
                }

                enums::Ailment::Poison => {
                    if !player.get_flags().contains_key(&enums::PlayerFlag::Safeguard) {
                        // Neither Poison nor steel pokemon can be poisoned in normal ways.
                        if target.get_types().0 == enums::Types::Poison ||
                           target.get_types().0 == enums::Types::Steel ||
                           target.get_types().1 == enums::Types::Poison ||
                           target.get_types().1 == enums::Types::Steel {
                            println!("{} could not be poisoned", target.get_name());
                        } else {
                            if name == "toxic" {
                                target.set_non_volatile(enums::NonVolatile::BadPoison);
                            } else {
                                target.set_non_volatile(enums::NonVolatile::Poison);
                            }
                        }
                    } else {
                        println!("{} was protected", target.get_name());
                    }
                }

                enums::Ailment::LeechSeed => {
                    // Has no effect on grass type (even though given the flavor text leech seeds
                    // are a plant parasite...)
                    if target.get_types().0 == enums::Types::Grass ||
                       target.get_types().1 == enums::Types::Grass {
                        println!("{} was not affected by Leech Seed", target.get_name());
                    } else {
                        target.add_end_flag(enums::EndOfTurn::LeechSeed);
                    }
                }

                enums::Ailment::PerishSong => {
                    // actually only one Attack, that kills all Pokemon after 4 rounds, including
                    // the user. Does not reset the counter if used again, therefore Pokemon, that
                    // are already under the effect of Perish Song are not influenced
                    if target.get_end_of_turn_flags().contains_key(&enums::EndOfTurn::PerishSong) {
                        println!("{} is already doomed", target.get_name());
                    } else {
                        target.add_end_flag(enums::EndOfTurn::PerishSong);
                    }
                }

                enums::Ailment::Yawn => {
                    if target.get_end_of_turn_flags().contains_key(&enums::EndOfTurn::Yawn) ||
                       target.get_non_volatile().0 == enums::NonVolatile::Sleep {
                        println!("{} was not affected by Yawn", target.get_name());
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
                        println!("{} was protected", target.get_name());
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
                        println!("{} was not affected by Attract", target.get_name());
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

// TODO: Methode implementieren, die errechnet wie viel ein Stage für das entsprechende Pokemon ist
// und den Stat entsprechend verringert/erhöht, wenn Stage 6/-6 noch nicht erreicht ist. Gibt einen
// bool zurück der anzeigt, ob der Stat verändert wurde oder nicht.
pub fn change_stats(stages: i8,
                    stat: enums::Stats,
                    target: &mut PokemonToken,
                    defender: &mut Player)
                    -> bool {
    if defender.get_flags().contains_key(&enums::PlayerFlag::Mist) {
        return false;
    }
    let mut current = target.get_current().get_stat(&stat);
    if target.get_non_volatile().0 == enums::NonVolatile::Paralysis {
        current = current * 2;
    }
    let stage = match stat {
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
    };
    println!("{:?}", stage);
    if !(stage <= -6 && stage >= 6) {
        let mut new_stage = stage + stages;
        if new_stage > 6 {
            new_stage = 6;
        } else if new_stage < -6 {
            new_stage = -6
        }
        let mut modifier = 1.0;
        if target.get_non_volatile().0 == enums::NonVolatile::Paralysis {
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
            println!("{}s {} cannot be lowered anymore",
                     target.get_name(),
                     enums::stat_to_string(stat));
        }
    }
    return false;
}


// Heals the targets HP by the provided value, or, if this would raise the HP above the base stat,
// to their base HP.
pub fn heal(target: &mut PokemonToken, value: u16) {
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
        println!("{} could not be healed", target.get_name());
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
    pokemon.get_current().set_stats(enums::Stats::Hp, clone.get_stat(&enums::Stats::Hp));
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
    if arena.get_current_weather() != weather {
        arena.set_current_weather(weather);
        arena.get_hash_weather().insert(weather, 0);
        return false;
    }
    true
}

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
