use super::pokemon_model;

#[derive(Debug, Clone)]
pub struct PokemonToken {
    pokedex_id: usize,
    name: String,
    gender: gender,
    type_one: pokemon_model::types,
    type_two: pokemon_model::types,
    base_stats: pokemon_model::stats,
    current_stats: pokemon_model::stats,
    mega_evolution: Box<Option<pokemon_model::PokemonModel>>,
}

impl PokemonToken {
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        PokemonToken {
            pokedex_id: model.pokedex_id,
            name: model.name,
            gender: gender::male,
            type_one: model.type_one,
            type_two: model.type_two,
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

#[derive(Debug, Clone)]
enum gender {
    male,
    female,
    genderless,
}
