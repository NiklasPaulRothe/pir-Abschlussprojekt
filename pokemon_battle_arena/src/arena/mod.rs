mod standard_arena;

use db::enums;
use player::Player;

pub struct Arena {
    effect: enums::types,
    weather: enums::Weather,
    team_1: Vec<Box<Player>>,
    team_2: Vec<Box<Player>>,
}

impl Arena {
    pub fn new(i_team_1: Vec<Box<Player>>, i_team_2: Vec<Box<Player>>, i_effect: enums::types,
        i_weather: enums::Weather) -> Self {
        Arena {
            effect: i_effect,
            weather: i_weather,
            team_1: i_team_1,
            team_2: i_team_2,
        }
    }
    // Getter Methods
    pub fn get_effect(&self) -> enums::types {
        self.effect.clone()
    }
    pub fn get_weather(&self) -> enums::Weather {
        self.weather.clone()
    }
    pub fn get_team_1_player(&self, player: usize) -> Option<&Box<Player>> {
        if player <= self.get_team_2_count() && player > 0 {
            return Some(&self.team_1[player - 1]);
        }
        None
    }
    pub fn get_team_2_player(&self, player: usize) -> Option<&Box<Player>> {
        if player <= self.get_team_2_count() && player > 0 {
            return Some(&self.team_2[player - 1]);
        }
        None
    }
    pub fn get_team_1_count(&self) -> usize {
        self.team_1.len()
    }
    pub fn get_team_2_count(&self) -> usize {
        self.team_2.len()
    }

    // Setter Methods
    pub fn set_effect(&mut self, new: enums::types) {
        self.effect = new;
    }
    pub fn set_weather(&mut self, new: enums::Weather) {
        self.weather = new;
    }
}
