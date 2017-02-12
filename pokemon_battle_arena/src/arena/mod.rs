pub mod standard_arena;
pub mod to_ui;

use db::enums;
use player::Player;

pub struct Arena<'a> {
    effect: enums::Types,
    weather: enums::Weather,
    player_1: &'a mut Player,
    player_2: &'a mut Player,
}

impl<'a> Arena<'a> {
    /// Creates a new arena with a list of players for both teams, the default effect and the
    /// default weather
    pub fn new(i_player_1: &'a mut Player, i_player_2: &'a mut Player, i_effect: enums::Types,
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
    #[allow(dead_code)]
    pub fn get_effect(&self) -> enums::Types {
        self.effect.clone()
    }
    /// Gets the actual weather of the arena
    #[allow(dead_code)]
    pub fn get_weather(&self) -> enums::Weather {
        self.weather.clone()
    }
    /// Returns a player one
    #[allow(dead_code)]
    pub fn get_player_one(&mut self) -> &mut Player {
        self.player_1
    }
    /// Returns a player two
    #[allow(dead_code)]
    pub fn get_player_two(&mut self) -> &mut Player {
        self.player_2
    }

    //
    // Setter Methods
    //
    /// Sets the effect of the arena
    #[allow(dead_code)]
    pub fn set_effect(&mut self, new: enums::Types) {
        self.effect = new;
    }
    /// Sets the weather in the arena
    #[allow(dead_code)]
    pub fn set_weather(&mut self, new: enums::Weather) {
        self.weather = new;
    }
}
