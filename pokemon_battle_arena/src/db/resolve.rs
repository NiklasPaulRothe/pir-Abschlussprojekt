extern crate rand;

use super::moves::Technique;
use super::pokemon_token::PokemonToken;
use self::rand::{Rng, thread_rng};

///Resolves moves that simply deals damage to the opponent.
pub fn deal_damage(attack: Technique, user: PokemonToken, target: PokemonToken) {
    unimplemented!();
    //TODO: Methode die matcht zwischen Attacken die direkt verrechnet werden können und denen,
    //die variable Power haben. Hier muss eine Möglichkeit gefunden werden die Power möglichst
    //effizient für alle Attacken zu berechnen.
}

pub fn ailment(attack: Technique, user: PokemonToken, target: PokemonToken) {
    let mut rng = thread_rng();
    let random = rng.gen_range(1, 101);
    let probability = attack.get_effect_chance();
    if random <= probability {
        match attack.get_ailment() {
            _ => {},
        }
    }
}

pub fn change_stats(attack: Technique, user: PokemonToken, target:PokemonToken) {
    unimplemented!();
}

pub fn heal(attack: Technique, user: PokemonToken, target: PokemonToken) {
    unimplemented!();
}
