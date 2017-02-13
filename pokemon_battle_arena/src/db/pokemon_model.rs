use super::enums;
use super::stats;

use enum_primitive::FromPrimitive;

/// Basic values for Pokemon species. Equal for every instance of the given Pokemon.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonModel {
    pokedex_id: usize,
    name: String,
    height: u8,
    weight: u16,
    gender_rate: i8,
    description: String,
    type_one: enums::Types,
    type_two: enums::Types,
    base_stats: stats::Stats,
    mega_evolution: Box<Option<PokemonModel>>,
}

impl PokemonModel {
    /// Creates a PokemonModel with the given arguments.
    pub fn new(id: usize,
               name: String,
               height: u8,
               weight: u16,
               gender_rate: i8,
               flavor_text: String)
               -> PokemonModel {
        PokemonModel {
            pokedex_id: id,
            name: name,
            height: height,
            weight: weight,
            gender_rate: gender_rate,
            description: flavor_text,
            type_one: enums::Types::from_i32(19).unwrap(),
            type_two: enums::Types::from_i32(19).unwrap(),
            base_stats: stats::Stats::default(),
            mega_evolution: Box::new(None),
        }
    }

    // Getter methods
    //
    /// Gets the pokedex id of the PokemonModel
    pub fn get_id(&self) -> usize {
        self.pokedex_id
    }
    /// Gets the name of the pokemon
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Gets the height of the pokemon
    pub fn get_height(&self) -> u8 {
        self.clone().height
    }
    /// Gets the weight of the pokemon
    pub fn get_weight(&self) -> u16 {
        self.clone().weight
    }
    /// Gets the gender rate of the pokemon
    pub fn get_gender_rate(&self) -> i8 {
        self.clone().gender_rate
    }
    /// Gets the description for the pokemon which is readable by the user of the program aswell
    pub fn get_description(&self) -> String {
        self.clone().description
    }
    /// Gets the types of the pokemon
    pub fn get_types(&self) -> (enums::Types, enums::Types) {
        (self.clone().type_one, self.clone().type_two)
    }
    /// Gets the base stats of the pokemon
    pub fn get_stats(&self) -> stats::Stats {
        self.clone().base_stats
    }
    /// Gets the possible mega evolution of the pokemon. None if there is no mega evolution
    pub fn get_mega(&self) -> Option<PokemonModel> {
        if self.has_mega() {
            return Some(self.clone().mega_evolution.unwrap());
        }
        None
    }
    // Setter methods
    //
    /// Sets a type of the pokemon
    pub fn set_type(&mut self, type_id: i32, slot: u16) {
        match slot {
            1 => self.type_one = enums::Types::from_i32(type_id).unwrap(),
            2 => self.type_two = enums::Types::from_i32(type_id).unwrap(),
            _ => unreachable!(),
        }
    }
    /// Sets the mega evolution of the pokemon
    pub fn set_mega(&mut self, model: PokemonModel) {
        self.mega_evolution = Box::new(Some(model));
    }
    /// Sets the stats of the pokemon
    pub fn set_stats(&mut self, stat_id: i32, value: u16) {
        self.base_stats.set_stats(enums::Stats::from_i32(stat_id).unwrap(), value);
    }
    // Other methods
    //
    /// Checks if the pokemon has a mega evolution
    pub fn has_mega(&self) -> bool {
        if self.mega_evolution.is_some() {
            return true;
        }
        false
    }
}
