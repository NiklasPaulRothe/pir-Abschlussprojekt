use db::pokedex::*;
use player::Player;
use db::pokemon_token::*;
use db::enums;

/// The representation of a human player
/// Stores e.g. the pokemon the player choose
pub struct Human {
    pokemon_list: Vec<PokemonToken>,
    pokemon_count: usize,
    current: usize,
}

impl Human {
    /// Creates a player by giving the id´s of the pokemon
// TO-DO: panic!´s if the id isnt in the range of the dex
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
            current: 0,
        }
    }
}

impl Player for Human {
    // Getter Methods
    fn get_pokemon_list(&self) -> &Vec<PokemonToken> {
        &self.pokemon_list
    }
    fn get_current(&self) -> usize {
        self.current
    }
    fn get_pokemon_count(&self) -> usize {
        self.pokemon_count
    }
    fn get_alive_count(&self) -> usize {
        self.pokemon_list.iter().filter(|x| x.get_current().get_stat(enums::Stats::Hp) != 0)
            .count()
    }
    fn get_alive_list(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        for i in 0..self.pokemon_list.len() {
            if self.pokemon_list[i].get_current().get_stat(enums::Stats::Hp) != 0 {
                vec.push(i);
            }
        }
        vec
    }
    // Setter Methods
    fn set_current(&mut self, new: usize) {
        self.current = new;
    }
}
