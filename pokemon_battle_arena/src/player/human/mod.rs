use db::pokemon_model::*;
use player::Player;

/// The representation of a human player
/// Stores e.g. the pokemon the player choose
pub struct Human {
    pokemon_list: Vec<PokemonModel>,
    pokemon_count: usize,
}

impl Human {
    /// Creates a player by giving the id´s of the pokemon
    pub fn new_by_id(input: &[usize]) -> Self {
        let mut pokemon = Vec::new();
        let len = input.len();
        for i in 0..input.len() {
            pokemon.push(pokemon_by_id(input[i]).unwrap());
        }

        Human {
            pokemon_list: pokemon,
            pokemon_count: len,
        }
    }
}

impl Player for Human {
    fn get_pokemon_list(&self) -> &Vec<PokemonModel> {
        &self.pokemon_list
    }

    fn get_pokemon_count(&self) -> &usize {
        &self.pokemon_count
    }
}
