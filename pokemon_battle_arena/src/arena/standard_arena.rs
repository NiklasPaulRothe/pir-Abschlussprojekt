use player::Player;
use db::pokemon_token::PokemonToken;
use db::enums;

/// The standard arena is based on the default 1v1 fight.

pub fn fight(mut player_one: &Player, mut player_two: &Player) {

    let mut p1_pokemon = &player_one.get_pokemon_list();
    let mut p2_pokemon = &player_two.get_pokemon_list();
    let mut p1_current = 0;
    let mut p2_current = 0;



    loop {
        if p1_pokemon[p1_current].get_current().get_stat(enums::Stats::Speed) >=
            p2_pokemon[p2_current].get_current().get_stat(enums::Stats::Speed) {
            // pokemon from p1 is faster and starts
            battle(&p1_pokemon[p1_current], &p2_pokemon[p2_current]);
        } else {
            // pokemon from p2 is faster and starts
            battle(&p2_pokemon[p2_current], &p1_pokemon[p1_current]);
        }
    }
}

/// Simulates one round in a fight
/// Every given pokemon doing one action.
fn battle(pokemon_one: &PokemonToken, pokemon_two: &PokemonToken) {

}
