use db::pokedex::*;
use player::{Player, AttackSlot};
use db::pokemon_token::*;
use db::{enums, moves};
use arena;


/// The representation of a human player
/// Stores e.g. the pokemon the player choose
pub struct Human {}