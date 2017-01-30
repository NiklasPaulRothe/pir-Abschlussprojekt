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
    name: String,
    gender: enums::Gender,
    type_one: enums::types,
    type_two: enums::types,
    nature: natures::Nature,
    dv: determinant_values::Dv,
    base_stats: stats::Stats,
    current_stats: stats::Stats,
    mega_evolution: Option<pokemon_model::PokemonModel>,
}


impl PokemonToken {
    ///Provides a Pokemon Token from a given model.
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        //TODO: Hier muss eine Methode aufgerufen werden, die die Stats für den Token errechnet und
        //das Ergebnis muss unten in den Struct geschrieben werden. Unter umständen müssen dafür die
        //DVs bereits zuvor errechnet werden, damit sie für die Berechnung herangezogen werden kön-
        //nen.

        PokemonToken {
            pokedex_id: model.get_id(),
            name: model.get_name(),
            gender: enums::get_gender(),
            type_one: model.get_types().0,
            type_two: model.get_types().1,
            nature: natures::Nature::get_random_nature(),
            dv: determinant_values::Dv::get_dv(model.clone()),
            base_stats: model.get_stats(),
            current_stats: model.get_stats(),
            mega_evolution: model.get_mega(),
        }
    }
    pub fn get_id(&self) -> usize {
        self.pokedex_id
    }

    pub fn get_name(&self) -> String {
        self.clone().name
    }

    pub fn get_gender(&self) -> enums::Gender {
        self.clone().gender
    }

    pub fn get_types(&self) -> (enums::types, enums::types) {
        (self.clone().type_one, self.clone().type_two)
    }

    pub fn get_nature(&self) -> natures::Nature {
        self.clone().nature
    }

    pub fn get_dv(&self) -> determinant_values::Dv {
        self.clone().dv
    }

    pub fn get_current(&self) -> stats::Stats {
        self.current_stats.clone()
    }

    pub fn get_base(&self) -> stats::Stats {
        self.base_stats.clone()
    }

    pub fn get_mega(&self) -> Option<PokemonToken> {
        if self.mega_evolution.is_some() {
            return Some(PokemonToken::from_model(self.mega_evolution.clone().unwrap()));
        }
        None
    }
}
