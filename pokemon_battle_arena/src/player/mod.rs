pub mod human;
pub mod ki;


use arena;
use db::pokemon_token::PokemonToken;
use db::pokedex::*;
use std::collections::HashMap;
use db::{self, moves, enums};

/// The Player type represents if the Player is a Human or a specific Ai to call different funcions
/// for e.g. choosing Pokemon
#[derive(Clone, Debug)]
pub enum PlayerType {
    Human,
    SimpleAi,
}

/// An enum which represents the AttackSlot to match with it
#[derive(Debug, Clone, Copy)]
pub enum AttackSlot {
    One,
    Two,
    Three,
    Four,
}
/// An enum representing the PokemonSlot
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonSlot {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
impl PokemonSlot {
    pub fn get_int(&self) -> usize {
        match *self {
            PokemonSlot::One => 1,
            PokemonSlot::Two => 2,
            PokemonSlot::Three => 3,
            PokemonSlot::Four => 4,
            PokemonSlot::Five => 5,
            PokemonSlot::Six => 6,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Next {
    Switch(PokemonSlot),
    Move(moves::Technique),
    Flinch,
    None,
}

/// The Player struct represents a Player and holds his Type (Human, Ai...), a list of his pokemon,
/// the amount of pokemon, his currently fighting pokemon and the next_move he wants to make
#[derive(Clone, Debug)]
pub struct Player {
    player: PlayerType,
    pokemon_list: Vec<PokemonToken>,
    pokemon_count: usize,
    current: usize,
    next_move: Option<Next>,
    flags: HashMap<enums::PlayerFlag, u8>,
    last_move: Option<moves::Technique>,
    last_action: (Next, u8),
    switched: bool,
}

impl Player {
    // Constructor
    //
    /// Takes an array with pokemon id´s and the PlayerType and returns a player
    pub fn new_by_id(input: &[usize], player_type: PlayerType) -> Self {
        let mut pokemon = Vec::new();
        let len = input.len();
        let dex = Pokedex::new();
        for i in 0..input.len() {
            pokemon.push(PokemonToken::from_model(dex.pokemon_by_id(input[i]).unwrap()));
        }
        Player {
            player: player_type,
            pokemon_list: pokemon,
            pokemon_count: len,
            current: 0,
            next_move: None,
            flags: HashMap::new(),
            last_move: None,
            last_action: (Next::None, 0),
            switched: false,
        }
    }

    pub fn new_by_pokemon(pokemon: Vec<db::pokemon_token::PokemonToken>,
                          player_type: PlayerType)
                          -> Self {
        Player {
            player: player_type,
            pokemon_list: pokemon.clone(),
            pokemon_count: pokemon.len(),
            current: 0,
            next_move: None,
            flags: HashMap::new(),
            last_move: None,
        }
    }
    // Getter methods
    //
    /// Returns the list of pokemon choosen by the player
    pub fn get_pokemon_list(&mut self) -> &mut Vec<PokemonToken> {
        &mut self.pokemon_list
    }
    /// Returns the currently fighting pokemon
    pub fn get_current(&self) -> usize {
        self.current
    }
    /// Gets the amount of pokemon choosen by the player
    pub fn get_pokemon_count(&self) -> usize {
        self.pokemon_count
    }
    /// Returns the amount of pokemon with atleast one hp
    pub fn get_alive_count(&mut self) -> usize {
        let mut alive = 0;
        for i in 0..self.pokemon_list.len() {
            if self.pokemon_list[i].get_current().get_stat(&enums::Stats::Hp) == 0 {
                alive += 1;
            }
        }
        alive
    }
    /// Returns a Vec with the id´s in the player model from the pokemon which are alive
    pub fn get_alive_list(&mut self) -> Vec<usize> {
        let mut vec = Vec::new();
        for i in 0..self.pokemon_list.len() {
            if self.get_pokemon_list()[i].get_current().get_stat(&enums::Stats::Hp) != 0 {
                vec.push(i);
            }
        }
        vec
    }
    /// Gets a attack saved in an attackslot
    pub fn get_attack(&mut self, slot: &AttackSlot) -> moves::Technique {
        match slot {
            &AttackSlot::One => self.pokemon_list[self.current].clone().get_move_one().unwrap(),
            &AttackSlot::Two => self.pokemon_list[self.current].clone().get_move_two().unwrap(),
            &AttackSlot::Three => self.pokemon_list[self.current].clone().get_move_three().unwrap(),
            &AttackSlot::Four => self.pokemon_list[self.current].clone().get_move_four().unwrap(),
        }
    }
    /// Checks if an attack is in the Attackslots and returns the Slot
    pub fn get_attack_slot(&mut self, attack: moves::Technique) -> Option<AttackSlot> {
        if self.get_attack(&AttackSlot::One) == attack {
            return Some(AttackSlot::One);
        } else if self.get_attack(&AttackSlot::Two) == attack {
            return Some(AttackSlot::Two);
        } else if self.get_attack(&AttackSlot::Three) == attack {
            return Some(AttackSlot::Three);
        } else if self.get_attack(&AttackSlot::Four) == attack {
            return Some(AttackSlot::Four);
        }
        None
    }
    /// Gets the next attack from the Player. Returns none if no Technique is selected
    pub fn get_next_move(&self) -> Option<Next> {
        self.next_move.clone()
    }
    /// Gets the last move with the Slot. Returns None if the last move wasn´t an attack
    pub fn get_last_move(&self) -> Option<moves::Technique> {
        self.last_move.clone()
    }

    pub fn get_last_action(&self) -> &(Next, u8) {
        &self.last_action
    }

    pub fn get_flags(&mut self) -> &mut HashMap<enums::PlayerFlag, u8> {
        &mut self.flags
    }

    pub fn get_switched(&mut self) -> bool {
        self.switched
    }

    // Setter methods
    //
    /// Sets the current value (e.g. after a pokemon swap)
    /// Given values should be between 1 and the maximum amount of pokemon you have
    pub fn set_current(&mut self, new: usize) {
        self.current = new;
    }
    /// Sets a next move to the Player
    pub fn set_next_move(&mut self, next: Option<Next>) {
        self.next_move = next;
    }
    pub fn add_flag(&mut self, flag: enums::PlayerFlag) {
        if !self.flags.contains_key(&flag) {
            self.flags.insert(flag, 0);
        } else {
            println!("It has no effect");
        }
    }
    /// Sets the last move the attacking pokemon made with the Slot in which it is saved
    pub fn set_last_move(&mut self, last: Option<moves::Technique>) {
        self.last_move = last;
    }

    /// Sets the last action
    pub fn set_last_action(&mut self, last: (Next, u8)) {
        self.last_action = last;
    }
    /// Sets the switched flag with a bool
    pub fn set_switched(&mut self, stat: bool) {
        self.switched = stat;
    }
    // Other
    //
    pub fn attack_or_swap(&self) -> arena::to_ui::Move {
        match self.player {
            PlayerType::Human => arena::to_ui::ui_move(),
            PlayerType::SimpleAi => arena::to_ui::ui_move(),
        }
    }
}
