use db::pokemon_token::*;
use player::Player;
use db::enums;

/// Representing a SimpleAi
pub struct SimpleAi {
    pokemon_list: Vec<PokemonToken>,
    pokemon_count: usize,
    current: usize,
}

impl Player for SimpleAi {
    // Getter Methods
    fn get_pokemon_list(&self) -> &Vec<PokemonToken> {
        &self.pokemon_list
    }

    fn get_pokemon_count(&self) -> usize {
        self.pokemon_count
    }
    fn get_current(&self) -> usize {
        self.current
    }
    fn get_alive_count(&self) -> usize {
        self.pokemon_list.iter().filter(|x| x.get_current().get_stat(enums::Stats::Hp) != 0).count()
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
        self.current = new-1;
    }
}
