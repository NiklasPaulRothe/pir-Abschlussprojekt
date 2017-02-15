pub mod standard_arena;
pub mod to_ui;

use std::collections::HashMap;
use db::enums;
use player::Player;

/// The Arena struct holds the weather and the type of the arena aswell as the references at the
/// two players
pub struct Arena<'a> {
    default_effect: enums::Types,
    current_effect: (enums::Types, u8),
    default_weather: enums::Weather,
    current_weather: (enums::Weather, u8),
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
            current_effect: (i_effect, 0),
            default_weather: i_weather,
            current_weather: (i_weather, 0),
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
    pub fn get_current_effect(&self) -> (enums::Types, u8) {
        self.current_effect
    }
    /// Gets the actual weather of the arena and the rounds its there
    pub fn get_current_weather(&self) -> (enums::Weather, u8) {
        self.current_weather
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
        self.current_effect = (new, 0);
    }
    /// Sets the current weather in the arena
    pub fn set_current_weather(&mut self, new: enums::Weather) {
        self.current_weather = (new, 0);
    }
    // Other Methods
    //
    /// Checks if field effects and weather are still valid.
    /// Field effects are compared with their maximum lifetime. If the effects reached it their
    /// dropped out of the HashMap
    /// Weather lasts for 5 rounds after it, it is replaced with the default arena weather.
    pub fn validate_effects_and_weather(&mut self) {
        // Validate field effects
        let mut tmp: HashMap<enums::FieldEffects, u8> = HashMap::new();
        for (effect, act_rounds) in self.get_field_effects().iter() {
            if act_rounds < &effect.get_max_rounds() {
                tmp.insert(*effect, act_rounds + 1);
            }
        }
        self.field_effects = tmp;
        // Validate weather effects
        if self.current_weather.1 < 4 {
            self.current_weather.1 += 1;
        } else {
            self.current_weather = (self.default_weather, 0);
        }
    }
}
