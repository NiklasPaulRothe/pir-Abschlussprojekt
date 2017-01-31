extern crate rand;

use super::moves::Technique;
use super::pokemon_token::PokemonToken;
use super::enums;
use self::rand::{Rng, thread_rng};
use player;

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
            enums::Ailment::Confusion => {},
            _ => {},
        }
    }
}

//TODO: Methode implementieren, die errechnet wie viel ein Stage für das entsprechende Pokemon ist
//und den Stat entsprechend verringert/erhöht, wenn Stage 6/-6 noch nicht erreicht ist. Gibt einen
//bool zurück der anzeigt, ob der Stat verändert wurde oder nicht.
pub fn change_stats(stages: i8, stat: enums::Stats, target: PokemonToken) -> bool {
    unimplemented!();
}

pub fn heal(attack: Technique, user: PokemonToken, target: PokemonToken) {
    unimplemented!();
}
