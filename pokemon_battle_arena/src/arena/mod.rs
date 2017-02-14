pub mod standard_arena;
pub mod to_ui;

use std::collections::HashMap;
use db::enums;
use player::Player;

/// The Arena struct holds the weather and the type of the arena aswell as the references at the
/// two players
pub struct Arena<'a> {
    default_effect: enums::Types,
    current_effect: enums::Types,
    effect_flag: HashMap<enums::Types, u8>,
    default_weather: enums::Weather,
    current_weather: enums::Weather,
    weather_flag: HashMap<enums::Weather, u8>,
    field_effects: HashMap<enums::FieldEffects, u8>,
    player_1: &'a mut Player,
    player_2: &'a mut Player,
}

impl<'a> Arena<'a> {
    /// Creates a new arena with a the two players, the default effect and the default weather
    pub fn new(i_player_1: &'a mut Player,
               i_player_2: &'a mut Player,
               i_effect: enums::Types,
               i_weather: enums::Weather)
               -> Self {
        Arena {
            default_effect: i_effect,
            current_effect: i_effect,
            effect_flag: HashMap::new(),
            default_weather: i_weather,
            current_weather: i_weather,
            weather_flag: HashMap::new(),
            field_effects: HashMap::new(),
            player_1: i_player_1,
            player_2: i_player_2,
        }
    }

    // Getter Methods
    //
    /// Gets the default type of the arena
    pub fn get_default_effect(&self) -> enums::Types {
        self.default_effect
    }
    /// Gets the default weather of the arena
    pub fn get_default_weather(&self) -> enums::Weather {
        self.default_weather
    }
    /// Gets the current type of the arena
    pub fn get_current_effect(&self) -> enums::Types {
        self.current_effect
    }
    /// Gets the actual weather of the arena
    pub fn get_current_weather(&self) -> enums::Weather {
        self.current_weather
    }
    /// Gets the effect flag list
    pub fn get_hash_effect(&mut self) -> &mut HashMap<enums::Types, u8> {
        &mut self.effect_flag
    }
    /// Gets the weather flag list
    pub fn get_hash_weather(&mut self) -> &mut HashMap<enums::Weather, u8> {
        &mut self.weather_flag
    }
    /// Gets the field effect list
    pub fn get_field_effects(&mut self) -> &mut HashMap<enums::FieldEffects, u8> {
        &mut self.field_effects
    }
    /// Returns a player one
    #[allow(dead_code)]
    pub fn get_player_one(&mut self) -> &mut Player {
        self.player_1
    }
    /// Returns a player two
    pub fn get_player_two(&mut self) -> &mut Player {
        self.player_2
    }

    // Setter Methods
    //
    /// Sets the current effect of the arena
    pub fn set_current_effect(&mut self, new: enums::Types) {
        self.current_effect = new;
    }
    /// Sets the current weather in the arena
    pub fn set_current_weather(&mut self, new: enums::Weather) {
        self.current_weather = new;
    }
    // Other Methods
    //
    /// Deletes all Field Effects which reached their maximum amount of rounds and increases the
    /// rounds for all remaining effects by one
    pub fn validate_field_effects(&mut self) {
        let mut tmp: HashMap<enums::FieldEffects, u8> = HashMap::new();
        for (effect, act_rounds) in self.get_field_effects().iter() {
            if act_rounds < &effect.get_max_rounds() {
                tmp.insert(*effect, act_rounds + 1);
            }
        }
        self.field_effects = tmp;
    }
}
