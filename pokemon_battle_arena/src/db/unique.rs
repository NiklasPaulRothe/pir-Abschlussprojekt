use time::get_time;
use player::Player;
use player::PlayerType;
use player;
use arena::Arena;
use super::enums;
use super::moves;
use super::resolve;
use super::pokemon_token::PokemonToken;
use super::movedex::Movedex;
use super::moves::Technique;
use self::rand::{Rng, thread_rng};
use self::regex::Regex;

extern crate rand;
extern crate regex;



#[derive(Debug)]
enum MoveName {
    disable,
    teleport
}

pub fn test(){
    println!("Unique");
    for entry in Movedex::new().get_entries() {
         if entry.get_category() == enums::MoveCategory::Unique {
             println!("{:?}: {:?} - {:?}", entry.get_id(), entry.get_name(), entry.get_type());
         }
    }
}

pub fn test2(){
    println!("Unique");
    for entry in Movedex::new().get_entries() {
         if entry.get_category() == enums::MoveCategory::Unique {
             println!("{:?}", entry.get_name());
         }
    }
}

pub fn unique(attack: &Technique, name: &str, move_type: enums::Types, mut user: PokemonToken, mut target: PokemonToken, attacker: &mut Player, defender: &mut Player, arena: &mut Arena) {
    let movedex = Movedex::new();
    let mut rng = thread_rng();

    // target.get_moves(dex.move_by_id());
    // match dex.get_entries() {
    match name {
        "disable" => {
            println!("disable" );
        },
        "teleport" => {
            println!("teleport");
        },
        "mimic" => {
            let attack = movedex.move_by_id(defender.get_last_move().unwrap().0.get_id()).unwrap();
            attack.resolve(arena, 1);
        },
        "focus-energy" => {
        },
        "metronome" => {
            let random = rng.gen_range(1, 616);
            let attack = movedex.move_by_id(random).unwrap();
            attack.resolve(arena, 1);
        },
        "mirror-move" => {
            let attack = movedex.move_by_id(defender.get_last_move().unwrap().0.get_id()).unwrap();
            attack.resolve(arena, 1);
        },
        "nature-power" => {
            match arena.get_current_effect().0 {
                enums::Types::Normal => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "tri-attack" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Flying => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "air-slash" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Ground => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "earth_power" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Rock => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "power_gem" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Fire => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "lava_plume" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Water => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "hydro_pump" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Grass => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "energy-ball" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Electric => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "thunderbolt" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                enums::Types::Ice => {
                    for entry in Movedex::new().get_entries() {
                        if entry.get_name() == "ice_beam" {
                            let attack = movedex.move_by_id(entry.get_id()).unwrap();
                            attack.resolve(arena, 1);
                        }
                    }
                },
                _ => {},
            }
        },
        // "transform" => {
        // },
        "splash" => {
            if arena.get_current_effect().0 == enums::Types::Flying{
                println!("The attack can not be used because {:?} is activated.", enums::Types::Flying);
            }else{
                println!("Nothing happens...");
            }
        },
        "rest" => {
            for entry in Movedex::new().get_entries() {
                if entry.get_name() == "rest" {
                    ////Fehler muss noch behofen werden
                    //let mut user_clone = moves::get_user(1, arena).clone();
                    // resolve::ailment("rest",
                    //                  entry.get_type(),
                    //                  entry.get_ailment(),
                    //                  100,
                    //                  user,
                    //                  &mut target,
                    //                  defender);
                }
            }

        },
        "conversion" => {
            user.set_type(0, attack.get_type());
        },
        "spite" => {
            // defender.get_last_move().unwrap().0.get_power_points();
            // target.decrement_ap();
        },
        "sleep-talk" => {
            struct AllMoves {
                one: String,
                two: String,
                three: String,
                four: String,
            }
            // let one: String;;
            let mut moves = AllMoves {one: "".to_string(), two: "".to_string(), three: "".to_string(), four: "".to_string(), };
            if user.is_asleep(){
                if attacker.get_attack(&player::AttackSlot::One).get_name() != "sleep-talk"{
                    // moves.one = attacker.get_attack(&player::AttackSlot::One).get_name();
                    moves.one = attacker.get_attack(&player::AttackSlot::One).get_name().to_string();
                }
                if attacker.get_attack(&player::AttackSlot::Two).get_name() != "sleep-talk"{
                    moves.two = attacker.get_attack(&player::AttackSlot::Two).get_name().to_string();
                }
                if attacker.get_attack(&player::AttackSlot::Three).get_name() != "sleep-talk"{
                    moves.three = attacker.get_attack(&player::AttackSlot::Three).get_name().to_string();
                }
                if attacker.get_attack(&player::AttackSlot::Four).get_name() != "sleep-talk"{
                    moves.four = attacker.get_attack(&player::AttackSlot::Four).get_name().to_string();
                }

                let mut random = rng.gen_range(1, 4);
                loop {
                    match random {
                        1 => {
                            if moves.one == "".to_string(){
                                random = rng.gen_range(2, 4);
                            }else{
                                let attack = movedex.move_by_id(attacker.get_attack(&player::AttackSlot::One).get_id()).unwrap();
                                attack.resolve(arena, 1);
                                break
                            }
                        },
                        2 => {
                            if moves.two == "".to_string(){
                                random = rng.gen_range(1, 4);
                            }else{
                                let attack = movedex.move_by_id(attacker.get_attack(&player::AttackSlot::Two).get_id()).unwrap();
                                attack.resolve(arena, 1);
                                break
                            }
                        },
                        3 => {
                            if moves.three == "".to_string(){
                                random = rng.gen_range(1, 4);
                            }else{
                                let attack = movedex.move_by_id(attacker.get_attack(&player::AttackSlot::Three).get_id()).unwrap();
                                attack.resolve(arena, 1);
                                break
                            }
                        },
                        4 => {
                            if moves.four == "".to_string(){
                                random = rng.gen_range(1, 3);
                            }else{
                                let attack = movedex.move_by_id(attacker.get_attack(&player::AttackSlot::Four).get_id()).unwrap();
                                attack.resolve(arena, 1);
                                break
                            }
                        },
                        _=> { random = rng.gen_range(1, 4); }
                    }
                }
            }
        },
        "celebrate" => {
            println!("{:?} disappears and then rises out of a birthday present that falls into the picture from above.", user.get_name());
        },
        "powder" => {
            if target.get_types().0 == enums::Types::Grass || target.get_types().1 == enums::Types::Grass{
                println!("{:?} has no effect on  Pokemon type plants", name);
            }
        },
        "reflect-type" => {
            user.set_type(0, target.get_types().0);
            user.set_type(1, target.get_types().1);
        },
        "soak" => {
            target.set_type(0, enums::Types::Water);
            target.set_type(1, enums::Types::Water);
        },
        // "conversion-2" => {
        // },
        // "substitute" => {
        // },
        // "sketch" => {
        // },
        // "spider-web" => {
        // },
        // "mind-reader" => {
        // },
        // "curse" => {
        // },
        // "protect" => {
        // },
        // "belly-drum" => {
        // },
        // "destiny-bond" => {
        // },
        // "detect" => {
        // },
        // "lock-on" => {
        // },
        // "endure" => {
        // },
        // "mean-look" => {
        // },
        // "heal-bell" => {
        // },
        // "pain-split" => {
        // },
        // "baton-pass" => {
        // },
        // "encore" => {
        // },
        // "psych-up" => {
        // },
        // "future-sight" => {
        // },
        // "stockpile" => {
        // },
        // "memento" => {
        // },
        // "follow-me" => {
        // },
        // "taunt" => {
        // },
        // "helping-hand" => {
        // },
        // "trick" => {
        // },
        // "role-play" => {
        // },
        // "wish" => {
        // },
        // "assist" => {
        // },
        // "magic-coat" => {
        // },
        // "recycle" => {
        // },
        // "skill-swap" => {
        // },
        // "imprison" => {
        // },
        // "refresh" => {
        // },
        // "grudge" => {
        // },
        // "snatch" => {
        // },
        // "camouflage" => {
        // },
        // "aromatherapy" => {
        // },
        // "block" => {
        // },
        // "doom-desire" => {
        // },
        // "healing-wish" => {
        // },
        // "acupressure" => {
        // },
        // "psycho-shift" => {
        // },
        // "power-trick" => {
        // },
        // "gastro-acid" => {
        // },
        // "copycat" => {
        // },
        // "power-swap" => {
        // },
        // "guard-swap" => {
        // },
        // "worry-seed" => {
        // },
        // "heart-swap" => {
        // },
        // "aqua-ring" => {
        // },
        // "magnet-rise" => {
        // },
        // "switcheroo" => {
        // },
        // "defog" => {
        // },
        // "lunar-dance" => {
        // },
        // "guard-split" => {
        // },
        // "power-split" => {
        // },
        // "rage-powder" => {
        // },
        // "simple-beam" => {
        // },
        // "entrainment" => {
        // },
        // "after-you" => {
        // },
        // "ally-switch" => {
        // },
        // "shell-smash" => {
        // },
        // "quash" => {
        // },
        // "bestow" => {
        // },
        // "trick-or-treat" => {
        // },
        // "forests-curse" => {
        // },
        // "topsy-turvy" => {
        // },
        // "flower-shield" => {
        // },
        // "electrify" => {
        // },
        // "kings-shield" => {
        // },
        // "spiky-shield" => {
        // },
        // "happy-hour" => {
        // },
        // "hold-hands" => {
        // },
        _ => {},
    };
}
