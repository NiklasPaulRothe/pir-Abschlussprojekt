use time::get_time;
use player::Player;
use player::PlayerType;
use arena::Arena;
use super::enums;
use super::moves;
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

// pub fn test_match(move_name: &str){
//     let dex = Movedex::new();
//     match move_name {
//         "disable" => {
//             println!("disable" );
//         },
//         "teleport" => {
//             println!("teleport");
//         },
//         "mimic" => {
//         },
//         "focus-energy" => {
//         },
//         "metronome" => {
//         },
//         "mirror-move" => {
//         },
//         "transform" => {
//         },
//         "splash" => {
//         },
//         "rest" => {
//         },
//         "conversion" => {
//         },
//         "substitute" => {
//         },
//         "sketch" => {
//         },
//         "spider-web" => {
//         },
//         "mind-reader" => {
//         },
//         "curse" => {
//         },
//         "conversion-2" => {
//         },
//         "spite" => {
//         },
//         "protect" => {
//         },
//         "belly-drum" => {
//         },
//         "destiny-bond" => {
//         },
//         "detect" => {
//         },
//         "lock-on" => {
//         },
//         "endure" => {
//         },
//         "mean-look" => {
//         },
//         "sleep-talk" => {
//         },
//         "heal-bell" => {
//         },
//         "pain-split" => {
//         },
//         "baton-pass" => {
//         },
//         "encore" => {
//         },
//         "psych-up" => {
//         },
//         "future-sight" => {
//         },
//         "stockpile" => {
//         },
//         "memento" => {
//         },
//         "follow-me" => {
//         },
//         "nature-power" => {
//         },
//         "taunt" => {
//         },
//         "helping-hand" => {
//         },
//         "trick" => {
//         },
//         "role-play" => {
//         },
//         "wish" => {
//         },
//         "assist" => {
//         },
//         "magic-coat" => {
//         },
//         "recycle" => {
//         },
//         "skill-swap" => {
//         },
//         "imprison" => {
//         },
//         "refresh" => {
//         },
//         "grudge" => {
//         },
//         "snatch" => {
//         },
//         "camouflage" => {
//         },
//         "aromatherapy" => {
//         },
//         "block" => {
//         },
//         "doom-desire" => {
//         },
//         "healing-wish" => {
//         },
//         "acupressure" => {
//         },
//         "psycho-shift" => {
//         },
//         "power-trick" => {
//         },
//         "gastro-acid" => {
//         },
//         "copycat" => {
//         },
//         "power-swap" => {
//         },
//         "guard-swap" => {
//         },
//         "worry-seed" => {
//         },
//         "heart-swap" => {
//         },
//         "aqua-ring" => {
//         },
//         "magnet-rise" => {
//         },
//         "switcheroo" => {
//         },
//         "defog" => {
//         },
//         "lunar-dance" => {
//         },
//         "guard-split" => {
//         },
//         "power-split" => {
//         },
//         "rage-powder" => {
//         },
//         "soak" => {
//         },
//         "simple-beam" => {
//         },
//         "entrainment" => {
//         },
//         "after-you" => {
//         },
//         "ally-switch" => {
//         },
//         "shell-smash" => {
//         },
//         "quash" => {
//         },
//         "reflect-type" => {
//         },
//         "bestow" => {
//         },
//         "trick-or-treat" => {
//         },
//         "forests-curse" => {
//         },
//         "topsy-turvy" => {
//         },
//         "flower-shield" => {
//         },
//         "electrify" => {
//         },
//         "kings-shield" => {
//         },
//         "spiky-shield" => {
//         },
//         "powder" => {
//         },
//         "happy-hour" => {
//         },
//         "celebrate" => {
//         },
//         "hold-hands" => {
//         },
//         _ => {},
//     };
// }

pub fn unique(attack: &Technique, name: &str, move_type: enums::Types, mut user: PokemonToken, target: &mut PokemonToken, attacker: &mut Player, defender: &mut Player, arena: &mut Arena) {
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

        },
        "focus-energy" => {
        },
        "metronome" => {
            // let random = rng.gen_range(1, 616);
            // let attack = movedex.move_by_id(random).unwrap();
            // attack.resolve(&mut arena, 1);
        },
        "mirror-move" => {
            defender.get_last_move();
        },
        // "transform" => {
        // },
        // "splash" => {
        // },
        // "rest" => {
        // },
        // "conversion" => {
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
        // "conversion-2" => {
        // },
        // "spite" => {
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
        // "nature-power" => {
        //     match field.get_effect() {
        //         enums::Types::Grass => {

        //         },
        //         enums::Types::Rock => {

        //         },
        //         enums::Types::Grass => {

        //         },
        //         enums::Types::Grass => {

        //         },
        //         enums::Types::Grass => {

        //         },
        //         enums::Types::Grass => {

        //         },
        //         enums::Types::Grass => {

        //         },
        //     }
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
