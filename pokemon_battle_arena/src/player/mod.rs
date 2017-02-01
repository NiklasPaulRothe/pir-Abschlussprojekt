pub mod human;
pub mod ki;

use db::pokemon_token::PokemonToken;

/// The Player Trait must be implemented by every sort of human players or ai´s
pub trait Player {
    /// Returns the list of pokemon choosen by the player
    fn get_pokemon_list(&self) -> &Vec<PokemonToken>;
    /// Gets the amount of pokemon choosen by the player
    fn get_pokemon_count(&self) -> usize;
    /// Returns the amount of pokemon with atleast one hp
    fn get_alive(&self) -> usize;
}


//Ignore this section, it's only a note for me which work needs to be done:

//moves: TODOs in resolve_effect method (2x Heal), is_asleep methode für PokemonToken

//TODO Artur: hits in moves.rs, change stats + deal damage in resolve.rs, Methode zum errechnen
//der stats in stats.rs
