pub mod standard_arena;
pub mod to_ui;

use db::enums;
use player::Player;
use std::borrow::BorrowMut;

pub struct Arena {
    effect: enums::types,
    weather: enums::Weather,
    player_1: Player,
    player_2: Player,
}

impl Arena {
    /// Creates a new arena with a list of players for both teams, the default effect and the
    /// default weather
    pub fn new(i_player_1: Player, i_player_2: Player, i_effect: enums::types,
        i_weather: enums::Weather) -> Self {
        Arena {
            effect: i_effect,
            weather: i_weather,
            player_1: i_player_1,
            player_2: i_player_2,
        }
    }

    //
    // Getter Methods
    //
    /// Gets the type of the arena
    pub fn get_effect(&self) -> enums::types {
        self.effect.clone()
    }
    /// Gets the actual weather of the arena
    pub fn get_weather(&self) -> enums::Weather {
        self.weather.clone()
    }
    /// Returns a player one
    pub fn get_player_one(&self) -> Player {
        self.player_1.clone()
    }
    /// Returns a player two
    pub fn get_player_two(&self) -> Player {
        self.player_2.clone()
    }

    //
    // Setter Methods
    //
    /// Sets the effect of the arena
    pub fn set_effect(&mut self, new: enums::types) {
        self.effect = new;
    }
    /// Sets the weather in the arena
    pub fn set_weather(&mut self, new: enums::Weather) {
        self.weather = new;
    }
}
