pub mod human;
pub mod ki;

use db::pokemon_model::PokemonModel;

/// The Player Trait must be implemented by every sort of human players or ai´s
pub trait Player {
    /// Returns the list of pokemon choosen by the player
    fn get_pokemon_list(&self) -> &Vec<PokemonModel>;
    /// Gets the amount of pokemon choosen by the player
    fn get_pokemon_count(&self) -> &usize;
}
