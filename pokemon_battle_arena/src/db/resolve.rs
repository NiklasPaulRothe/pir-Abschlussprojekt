use super::moves::Technique;
use super::pokemon_token::PokemonToken;

///Resolves moves that simply deals damage to the opponent.
pub fn deal_damage(attack: Technique, user: PokemonToken, target: PokemonToken) {
    unimplemented!();
    //TODO: Methode die matcht zwischen Attacken die direkt verrechnet werden können und denen,
    //die variable Power haben. Hier muss eine Möglichkeit gefunden werden die Power möglichst
    //effizient für alle Attacken zu berechnen.
}
