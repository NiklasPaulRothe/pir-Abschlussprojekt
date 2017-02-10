use player::Player;
use db::pokemon_token::PokemonToken;
use db::enums;
use player::AttackSlot;

pub enum Move {
    Swap,
    Attack,
}


pub fn ui_move() -> Move {
    Move::Swap
}

pub fn ui_swap(pokemon: &Vec<PokemonToken>, current: usize) -> usize {
    current
}

pub fn ui_attack(pokemon: PokemonToken) -> AttackSlot {
    AttackSlot::One
}
