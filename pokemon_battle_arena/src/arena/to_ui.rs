// use db::pokemon_token::PokemonToken;
// use player::AttackSlot;

#[allow(dead_code)]
pub enum Move {
    Swap,
    Attack,
}

#[allow(dead_code)]
pub fn ui_move() -> Move {
    Move::Swap
}
// #[allow(dead_code)]
// pub fn ui_swap(pokemon: &Vec<PokemonToken>, current: usize) -> usize {
//     current
// }
// #[allow(dead_code)]
// pub fn ui_attack(pokemon: PokemonToken) -> AttackSlot {
//     AttackSlot::One
// }
