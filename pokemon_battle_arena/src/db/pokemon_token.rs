extern crate csv;

use super::pokemon_model;
use super::natures;
use super::enums;
use super::stats;
use super::determinant_values;

///Represents a single Token of a Pokemon with individual values for this token.
#[derive(Debug, Clone)]
pub struct PokemonToken {
    pokedex_id: usize,
    pub name: String,
    gender: enums::Gender,
    pub type_one: enums::types,
    pub type_two: enums::types,
    nature: natures::Nature,
    pub dv: determinant_values::Dv,
    base_stats: stats::Stats,
    current_stats: stats::Stats,
    mega_evolution: Box<Option<pokemon_model::PokemonModel>>,
}


impl PokemonToken {
    ///Provides a Pokemon Token from a given model.
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        //TODO: Hier muss eine Methode aufgerufen werden, die die Stats für den Token errechnet und
        //das Ergebnis muss unten in den Struct geschrieben werden. Unter umständen müssen dafür die
        //DVs bereits zuvor errechnet werden, damit sie für die Berechnung herangezogen werden kön-
        //nen.

        PokemonToken {
            pokedex_id: model.pokedex_id,
            name: model.clone().name,
            gender: enums::get_gender(),
            type_one: model.clone().type_one,
            type_two: model.clone().type_two,
            nature: natures::Nature::get_random_nature(),
            dv: determinant_values::Dv::get_dv(model.clone()),
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

    pub fn get_current(&self) -> stats::Stats {
        self.current_stats.clone()
    }
}
