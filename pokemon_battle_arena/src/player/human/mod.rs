use db::pokedex::*;
use player::Player;
use db::pokemon_token::*;
use db::enums;

/// The representation of a human player
/// Stores e.g. the pokemon the player choose
pub struct Human {
    pokemon_list: Vec<PokemonToken>,
    pokemon_count: usize,
}

impl Human {
    /// Creates a player by giving the id´s of the pokemon
    pub fn new_by_id(input: &[usize]) -> Self {
        let mut pokemon = Vec::new();
        let len = input.len();
        let dex = Pokedex::new();
        for i in 0..input.len() {
            pokemon.push(PokemonToken::from_model(dex.pokemon_by_id(input[i]).unwrap()));
        }

        Human {
            pokemon_list: pokemon,
            pokemon_count: len,
        }
    }
}

impl Player for Human {
    fn get_pokemon_list(&self) -> &Vec<PokemonToken> {
        &self.pokemon_list
    }
    fn get_pokemon_count(&self) -> usize {
        self.pokemon_count
    }
    fn get_alive(&self) -> usize {
        self.pokemon_list.iter().filter(|x| x.get_current().get_stat(enums::Stats::Hp) != 0).count()
    }
}
