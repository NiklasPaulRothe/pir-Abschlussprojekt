use time::get_time;
use player::Player;
use player::PlayerType;
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

pub fn unique(attack: &Technique, name: &str, move_type: enums::Types, mut user: PokemonToken, target: PokemonToken, attacker: &mut Player, defender: &mut Player, arena: &mut Arena) {
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
            match arena.get_effect() {
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
            if arena.get_effect() == enums::Types::Flying{
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
                    //                  target,
                    //                  defender);
                }
            }

        },
        "conversion" => {
            user.set_type(0, attack.get_type());
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
        "spite" => {
            user.set_type(0, attack.get_type());
            defender.get_last_move().unwrap().0.get_power_points()
        },
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
        // "sleep-talk" => {
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
        // "soak" => {
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
        // "reflect-type" => {
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
        // "powder" => {
        // },
        // "happy-hour" => {
        // },
        // "celebrate" => {
        // },
        // "hold-hands" => {
        // },
        _ => {},
    };
}
