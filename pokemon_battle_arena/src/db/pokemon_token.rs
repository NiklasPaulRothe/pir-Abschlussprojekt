use super::pokemon_model;
use super::natures;

#[derive(Debug, Clone)]
pub struct PokemonToken {
    pokedex_id: usize,
    name: String,
    gender: Gender,
    pub type_one: pokemon_model::types,
    pub type_two: pokemon_model::types,
    nature: natures::Nature,
    base_stats: pokemon_model::stats,
    current_stats: pokemon_model::stats,
    mega_evolution: Box<Option<pokemon_model::PokemonModel>>,
}

#[derive(Debug, Clone)]
enum Gender {
    Male,
    Female,
    Genderless,
}

impl PokemonToken {
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        PokemonToken {
            pokedex_id: model.pokedex_id,
            name: model.name,
            gender: Gender::Male,
            type_one: model.type_one,
            type_two: model.type_two,
            nature: natures::get_random_nature(),
            base_stats: model.base_stats.clone(),
            current_stats: model.base_stats,
            mega_evolution: model.mega_evolution,
        }
    }

    pub fn get_mega(&self) -> Option<PokemonToken> {
        if self.mega_evolution.is_some() {
            return Some(PokemonToken::from_model(self.mega_evolution.clone().unwrap()));
        }
        None
    }
}
