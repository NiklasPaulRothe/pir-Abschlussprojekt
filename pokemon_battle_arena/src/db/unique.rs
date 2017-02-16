use player;
use player::Player;
use arena::Arena;
use super::enums;
use super::moves::Technique;
use super::resolve;
use super::pokemon_token::PokemonToken;
use super::movedex::Movedex;
//use super::moves;
use self::rand::{Rng, thread_rng};
use graphic;

extern crate rand;

pub fn unique(attack: &Technique,
              name: &str,
              mut user: PokemonToken,
              mut target: PokemonToken,
              attacker: &mut Player,
              defender: &mut Player,
              arena: &mut Arena,
              flag: enums::Player,
              mut window: &mut graphic::gui::App) {
    let movedex = Movedex::new();
    let mut rng = thread_rng();

    // target.get_moves(dex.move_by_id());
    // match dex.get_entries() {
    match name {
        "disable" => {
            println!("disable");
        }
        "teleport" => {
            resolve::switch_pokemon(attacker);
        }
        "mimic" => {
            let attack = movedex.move_by_id(defender.get_last_move().unwrap().get_id())
                .unwrap();
            attack.resolve(arena, flag, window);
        }
        "focus-energy" => {}
        "metronome" => {
            let random = rng.gen_range(1, 616);
            let attack = movedex.move_by_id(random).unwrap();
            attack.resolve(arena, flag, window);
        }
        "mirror-move" => {
            let attack = movedex.move_by_id(defender.get_last_move().unwrap().get_id())
                .unwrap();
            attack.resolve(arena, flag, window);
        }
        "nature-power" => {
            match arena.get_current_effect().0 {
                enums::Types::Normal => {
                    //"tri-attack"
                    let attack = movedex.move_by_id(161).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Flying => {
                    //"air-slash" {
                    let attack = movedex.move_by_id(403).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Ground => {
                    //"earth-power" {
                    let attack = movedex.move_by_id(414).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Rock => {
                    //"power_gem" {
                    let attack = movedex.move_by_id(408).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Fire => {
                    //"lava-plume" {
                    let attack = movedex.move_by_id(436).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Water => {
                    //"hydro-pump" {
                    let attack = movedex.move_by_id(56).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Grass => {
                    //"energy-ball" {
                    let attack = movedex.move_by_id(412).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Electric => {
                    //"thunderbolt" {
                    let attack = movedex.move_by_id(85).unwrap();
                    attack.resolve(arena, flag, window);
                }
                enums::Types::Ice => {
                    //"ice-beam" {
                    let attack = movedex.move_by_id(58).unwrap();
                    attack.resolve(arena, flag, window);
                }
                _ => {}
            }
        }
        // "transform" => {
        // },
        "splash" => {
            //ob flag für gravity gesetz ist
            if arena.get_current_effect().0 == enums::Types::Flying {
                println!("The attack can not be used because {:?} is activated.",
                         enums::Types::Flying);
            } else {
                println!("Nothing happens...");
            }
        }
        "rest" => {
            for entry in Movedex::new().get_entries() {
                if entry.get_name() == "rest" {
                    ////Fehler muss noch behofen werden
                    //let mut user_clone = moves::get_user(flag, arena).clone();
                    // resolve::ailment("rest",
                    //                  entry.get_type(),
                    //                  entry.get_ailment(),
                    //                  100,
                    //                  user,
                    //                  &mut target,
                    //                  defender,
                    //                  window);
                }
            }

        }
        "conversion" => {
            user.set_type(0, attack.get_type());
        }
        "spite" => {
            defender.get_last_move().unwrap().get_power_points();
            target.decrement_ap();
        }
        "sleep-talk" => {
            if user.is_asleep() {
                let mut id_vec = Vec::new();
                if attacker.get_attack(&player::AttackSlot::One).get_name() != "sleep-talk" {
                    id_vec.push(attacker.get_attack(&player::AttackSlot::One).get_id());
                }
                if attacker.get_attack(&player::AttackSlot::Two).get_name() != "sleep-talk" {
                    id_vec.push(attacker.get_attack(&player::AttackSlot::Two).get_id());
                }
                if attacker.get_attack(&player::AttackSlot::Three).get_name() != "sleep-talk" {
                    id_vec.push(attacker.get_attack(&player::AttackSlot::Three).get_id());
                }
                if attacker.get_attack(&player::AttackSlot::Four).get_name() != "sleep-talk" {
                    id_vec.push(attacker.get_attack(&player::AttackSlot::Four).get_id());
                }
                let random = rng.gen_range(0, id_vec.len());
                let attack = movedex.move_by_id(id_vec[random]).unwrap();
                attack.resolve(arena, flag, window);
            }
        }
        "celebrate" => {
            println!("{:?}",
                     user.get_name() + "disappears and then rises out of a birthday" +
                     "present that falls into the picture from above.");
        }
        "powder" => {
            //resolv in db enams
            if target.get_types().0 == enums::Types::Grass ||
               target.get_types().1 == enums::Types::Grass {
                println!("{:?} has no effect on  Pokemon type plants", name);
            }
        }
        "reflect-type" => {
            user.set_type(0, target.get_types().0);
            user.set_type(1, target.get_types().1);
        }
        "soak" => {
            target.set_type(0, enums::Types::Water);
            target.set_type(1, enums::Types::Water);
        }
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
        _ => {}
    };
}
