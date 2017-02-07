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

//resolves ailment effects
pub fn ailment(name: String, move_type: enums::types, ailment: enums::Ailment, effect_chance: u8,
    mut target: PokemonToken) {
    let mut rng = thread_rng();
    let random = rng.gen_range(1, 101);
    //only works if the effect chance of the move is met.
    let probability = effect_chance;
    if random <= probability {
        let powder = Regex::new(r"powder").unwrap();
        let spore = Regex::new(r"spore").unwrap();
        let tmp: &str = & name;
        //some sort of attacks did not work against grass types.
        if (target.get_types().0 == enums::types::grass ||
            target.get_types().1 == enums::types::grass) && (powder.is_match(tmp)
            || spore.is_match(tmp)) {
            println!("{} was not affected by {}", target.get_name(), name);
        } else {
            //categorize the moves by the ailment they cause. Ailments usually automatically fail
            //if the target already was hit by a move that caused the same ailment and still suffer
            //from it's effect. Non volatile Ailments even fail if the target is under the effect
            //of one of these kind.
            match ailment {

                enums::Ailment::Paralysis => {
                    //electric type pokemon are immune to paralysis
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
                        println!("{} is already {}", target.get_name(),
                            enums::print_non_volatile(target.get_non_volatile().0));
                    }
                },

                enums::Ailment::Freeze => {
                    //ice type pokemon are immune to freeze, but only if the used move is also
                    //from the type ice.
                    if (target.get_types().0 == enums::types::ice || target.get_types().1 ==
                    enums::types::ice) && move_type == enums::types::ice {
                        println!("{} could not be freezed", target.get_name());
                    } else {
                        target.set_non_volatile(enums::Non_Volatile::Freeze);
                    }
                },

                enums::Ailment::Burn => {
                    //Fire types can not be burned (seems logical).
                    if target.get_types().0 == enums::types::fire || target.get_types().1 ==
                    enums::types::fire {
                        println!("{} could not be burned", target.get_name());
                    } else {
                        target.set_non_volatile(enums::Non_Volatile::Burn);
                    }
                },

                enums::Ailment::Poison => {
                    //Neither Poison nor steel pokemon can be poisoned in normal ways.
                    if target.get_types().0 == enums::types::poison || target.get_types().0 ==
                    enums::types::steel || target.get_types().1 == enums::types::poison ||
                    target.get_types().1 == enums::types::steel {
                        println!("{} could not be poisoned", target.get_name());
                    } else {
                        if name == String::from("toxic") {
                            target.set_non_volatile(enums::Non_Volatile::Bad_Poison);
                        } else {
                            target.set_non_volatile(enums::Non_Volatile::Poison);
                        }
                    }
                },

                enums::Ailment::Leech_Seed => {
                    //Has no effect on grass type (even though given the flavor text leech seeds
                    //are a plant parasite...)
                    if target.get_types().0 == enums::types::grass || target.get_types().1 ==
                    enums::types::grass {
                        println!("{} was not affected by Leech Seed", target.get_name());
                    } else {
                        target.add_end_flag(enums::End_Of_Turn::Leech_Seed);
                    }
                },

                enums::Ailment::Perish_Song => {
                    //actually only one Attack, that kills all Pokemon after 4 rounds, including
                    //the user. Does not reset the counter if used again, therefore Pokemon, that
                    //are already under the effect of Perish Song are not influenced
                    if target.get_end_of_turn_flags().contains_key(&enums::End_Of_Turn::Perish_Song) {
                        println!("{} is already doomed", target.get_name());
                    } else {
                        target.add_end_flag(enums::End_Of_Turn::Perish_Song);
                    }
                },

                enums::Ailment::Yawn => {
                    if target.get_end_of_turn_flags().contains_key(&enums::End_Of_Turn::Yawn) ||
                    target.get_non_volatile().0 == enums::Non_Volatile::Sleep {
                        println!("{} was not affected by Yawn", target.get_name());
                    } else {
                        target.add_end_flag(enums::End_Of_Turn::Yawn);
                    }
                },

                enums::Ailment::Trap => {
                    if !target.get_end_of_turn_flags().contains_key(&enums::End_Of_Turn::Trap) {
                        target.add_end_flag(enums::End_Of_Turn::Trap);
                    }
                }

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

//heals the targets HP by the provided value, or, if this would raise the HP above the base stat,
//to their base HP.
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

//switches the Pokemon of the target Player
pub fn switch_pokemon<T> (mut target: T)
    where T: Player + Clone {
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

//simply sets the HP of the target to 0 (Thats what K.O. means I suppose.)
pub fn ko_attack (target: PokemonToken) {
    target.get_current().set_stats(enums::Stats::Hp, 0);
}
