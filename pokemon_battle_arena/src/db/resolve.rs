extern crate rand;
extern crate regex;

use super::moves::Technique;
use super::pokemon_token::PokemonToken;
use super::enums;
use self::rand::{Rng, thread_rng};
use self::regex::Regex;
use player::Player;

///Resolves moves that simply deals damage to the opponent.
pub fn deal_damage(attack: Technique, user: PokemonToken, target: PokemonToken) -> u16 {
    unimplemented!();
    //TODO: Methode die matcht zwischen Attacken die direkt verrechnet werden können und denen,
    //die variable Power haben. Hier muss eine Möglichkeit gefunden werden die Power möglichst
    //effizient für alle Attacken zu berechnen.
}

pub fn ailment(name: String, move_type: enums::types, ailment: enums::Ailment, effect_chance: u8,
    mut target: PokemonToken) {
    let mut rng = thread_rng();
    let random = rng.gen_range(1, 101);
    let probability = effect_chance;
    if random <= probability {
        let powder = Regex::new(r"powder").unwrap();
        let spore = Regex::new(r"spore").unwrap();
        let tmp: &str = & name;
        if (target.get_types().0 == enums::types::grass ||
            target.get_types().1 == enums::types::grass) && (powder.is_match(tmp)
            || spore.is_match(tmp)) {
            println!("{} was not affected by {}", target.get_name(), name);
        } else {
            match ailment {
                enums::Ailment::Paralysis => {
                    if target.get_non_volatile().0 == enums::Non_Volatile::Undefined {
                        if !(target.get_types().0 == enums::types::electric) &&
                        !(target.get_types().1 == enums::types::electric) {
                            target.set_non_volatile(enums::Non_Volatile::Paralysis);
                            target.get_current().set_stats(enums::Stats::Speed, target.get_base().
                                get_stat(enums::Stats::Speed) / 2)
                        } else {
                            println!("{} was not affected by {}", target.get_name(), name);
                        }
                    } else {
                        println!("{} is already {}", target.get_name(),
                            enums::print_non_volatile(target.get_non_volatile().0));
                    }
                },
                enums::Ailment::Sleep => {
                    if target.get_non_volatile().0 == enums::Non_Volatile::Undefined {
                        target.set_non_volatile(enums::Non_Volatile::Sleep)
                    } else {
                        println!("{} is alreadey {}", target.get_name(),
                            enums::print_non_volatile(target.get_non_volatile().0));
                    }
                },
                enums::Ailment::Freeze => {
                    if (target.get_types().0 == enums::types::ice || target.get_types().1 ==
                    enums::types::ice) && move_type == enums::types::ice {
                        println!("{} could not be freezed", target.get_name());
                    } else {
                        target.set_non_volatile(enums::Non_Volatile::Freeze);
                    }
                },
                enums::Ailment::Burn => {
                    if target.get_types().0 == enums::types::fire || target.get_types().1 ==
                    enums::types::fire {
                        println!("{} could not be burned", target.get_name());
                    } else {
                        target.set_non_volatile(enums::Non_Volatile::Burn);
                    }
                },
                enums::Ailment::Poison => {
                    if target.get_types().0 == enums::types::poison || target.get_types().0 ==
                    enums::types::steel || target.get_types().1 == enums::types::poison ||
                    target.get_types().1 == enums::types::steel {
                        println!("{} could not be poisoned", target.get_name());
                    } else {
                        if name == String::from("toxic") {
                            target.set_non_volatile(enums::Non_Volatile::Bad_Poison);
                            //TODO: add counter for damage in badly poison state.
                        } else {
                            target.set_non_volatile(enums::Non_Volatile::Poison);
                        }
                    }
                },
                _ => {},
            }
        }
    }
}

//TODO: Methode implementieren, die errechnet wie viel ein Stage für das entsprechende Pokemon ist
//und den Stat entsprechend verringert/erhöht, wenn Stage 6/-6 noch nicht erreicht ist. Gibt einen
//bool zurück der anzeigt, ob der Stat verändert wurde oder nicht.
pub fn change_stats(stages: i8, stat: enums::Stats, target: PokemonToken) -> bool {
    true
}

pub fn heal(target: PokemonToken, value: u16) {
    if value + target.get_current().get_stat(enums::Stats::Hp) >= target.get_base().
    get_stat(enums::Stats::Hp) {
        target.get_current().set_stats(enums::Stats::Hp, target.get_base().
            get_stat(enums::Stats::Hp));
    } else {
        target.get_current().set_stats(enums::Stats::Hp, (target.get_current().
            get_stat(enums::Stats::Hp) + value));
    }
}

pub fn switch_pokemon<Player> (target: Player) {
        unimplemented!();
}

pub fn ko_attack (target: PokemonToken) {
    target.get_current().set_stats(enums::Stats::Hp, 0);
}
