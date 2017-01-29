extern crate rand;

use self::rand::{Rng, thread_rng};
use super::pokemon_model;

#[derive(Debug, Clone)]
pub struct Dv {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
}

impl Dv {
    pub fn get_dv(poke: pokemon_model::PokemonModel) -> Dv {
        let mut rng = thread_rng();
        let mut sample = Vec::new();
        let count = 0;
        // if poke.is_legendary {
        //     for _ in 0..3 {
        //         sample.push(31);
        //     }
        //     count = 2;
        // }
        for _ in count..6 {
            sample.push(rng.gen_range(0, 32))
        }
        rng.shuffle(&mut sample);
        Dv {
            hp: sample[0],
            attack: sample[1],
            defense: sample[2],
            special_attack: sample[3],
            special_defense: sample[4],
            speed: sample[5],
        }
    }
}
