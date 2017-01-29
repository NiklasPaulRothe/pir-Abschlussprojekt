use db::pokemon_token::*;
use player::Player;

/// Representing a SimpleAi
pub struct SimpleAi {
    pokemon_list: Vec<PokemonToken>,
    pokemon_count: usize,
}

impl Player for SimpleAi {
    fn get_pokemon_list(&self) -> &Vec<PokemonToken> {
        &self.pokemon_list
    }

    fn get_pokemon_count(&self) -> usize {
        self.pokemon_count
    }
    fn get_alive(&self) -> usize {
        self.pokemon_list.iter().filter(|x| x.get_current().hp != 0).count()
    }
}
