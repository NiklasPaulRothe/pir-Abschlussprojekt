extern crate rand;

use std::collections::HashMap;
use player::Next;
use db::{enums, moves};
use graphic;



/// The standard arena is based on the default 1v1 fight.

impl<'a> super::Arena<'a> {
    /// Simulating a fight. This function is called on a arena and uses the next_move variables of
    /// the players to know what to do in this round.
    /// Important: All next_move variables must contain a Some() entry. If the function is called
    /// and atleast one variable is holding a None, this function will panic!
    pub fn fight(&mut self, mut window: &mut graphic::gui::App) {

        // This flag is used to show that the round is "over" earlier as aspected.
        // This can be happen if pursuit was used or both pokemons are swapped.
        let mut end_of_fight = false;
        // Setting the switched flag in the Player structs to false and reset if a swap will be done
        //
        self.get_player_one().set_switched(false);
        self.get_player_two().set_switched(false);

        // Handle the pursuit(ID: 228) attack
        //
        match self.get_player_one()
            .get_next_move()
            .expect("Unexpected error! This field of player one shouldn`t be None at this point.") {
            Next::Move(technique) => {
                if technique.get_id() == 228 {
                    match self.get_player_one()
                        .get_next_move()
                        .expect("Unexpected error! This field of player one shouldn`t be None \
                                 at this point.") {
                        Next::Switch(_) => {
                            // Resolving pursuit, updating last action and last move
                            // and setting the next move to None
                            call_resolve(self, technique, enums::Player::Two, &mut window);
                            // let slot =
                            //     self.get_player_one().get_attack_slot(technique.clone())
                            //         .unwrap();
                            // self.get_player_one().set_last_move(Some((technique, slot)));
                            // let old_move = self.get_player_one().get_next_move().unwrap()
                            //         .clone();
                            // self.get_player_one().set_last_action(old_move);
                            self.get_player_one().set_next_move(None);

                        }
                        _ => {}
                    }
                }
            }
            Next::Switch(_) => {
                match self.get_player_one()
                    .get_next_move()
                    .expect("Unexpected error! This field of player one shouldn`t be None at \
                             this point.") {
                    Next::Move(technique) => {
                        if technique.get_id() == 228 {
                            // Resolving pursuit, updating last action and last move
                            // and setting the next move to None
                            call_resolve(self, technique, enums::Player::One, &mut window);
                            // let slot =
                            //     self.get_player_two().get_attack_slot(technique.clone())
                            //         .unwrap();
                            // self.get_player_two().set_last_move(Some((technique, slot)));
                            // let old_move = self.get_player_two().get_next_move().unwrap()
                            //         .clone();
                            // self.get_player_two().set_last_action(old_move);
                            self.get_player_two().set_next_move(None);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        // Switch Pokemon of Player One if he wants to
        //
        if let Some(x) = self.get_player_one().get_next_move() {
            match x {
                Next::Switch(pkmn) => {
                    // Switch of the current pokemon + setting flag
                    self.get_player_one().set_current(pkmn.get_int());
                    self.get_player_one().set_switched(true);
                    // Updating last action and setting next move to None. Last Move isnt updated
                    // because the last action wasnt a move
                    let old_move = self.get_player_one().get_next_move().unwrap().clone();
                    self.get_player_one().set_last_action((old_move, 0));
                    self.get_player_one().set_next_move(None);
                }
                _ => {}
            }
        }
        // Switch Pokemon of Player Two if he wants to
        //
        if let Some(x) = self.get_player_two().get_next_move() {
            match x {
                Next::Switch(pkmn) => {
                    // Switch of the current pokemon + setting flag
                    self.get_player_two().set_current(pkmn.get_int());
                    self.get_player_two().set_switched(true);
                    // Updating last action and setting next move to None. Last Move isnt updated
                    // because the last action wasnt a move
                    let old_move = self.get_player_two().get_next_move().unwrap().clone();
                    self.get_player_two().set_last_action((old_move, 0));
                    self.get_player_two().set_next_move(None);
                }
                _ => {}
            }
        }
        // If player one doesnt need to make a move anymore, only resolve attack of player two
        //
        if self.get_player_one().get_next_move().is_none() &&
           self.get_player_two().get_next_move().is_some() {
            match self.get_player_two().get_next_move().unwrap() {
                Next::Move(x) => call_resolve(self, x, enums::Player::Two, &mut window),
                _ => {}
            }
            end_of_fight = true;
            // If player two doesnt need to make a move anymore, only resolve attack of player one
            //
        } else if self.get_player_two().get_next_move().is_none() &&
                  self.get_player_one().get_next_move().is_some() {
            match self.get_player_one().get_next_move().unwrap() {
                Next::Move(x) => call_resolve(self, x, enums::Player::One, &mut window),
                _ => {}
            }
            end_of_fight = true;
            // If both player dont have a move go out of fight
            //
        } else if self.get_player_two().get_next_move().is_none() &&
                  self.get_player_one().get_next_move().is_none() {
            end_of_fight = true;
        }


        // If both player want to perform an attack Priority and Speed of Pokemon will be used to
        // decide which pokemon strikes first
        //
        // Variables for faster comparison. x_prio is the priority of the pokemon of player x and
        // x_speed is the attackspeed of the pokemon of player x
        if !end_of_fight {
            let one_prio;
            let one_attack;
            match self.get_player_one().get_next_move().unwrap() {
                Next::Move(attack) => {
                    one_prio = attack.get_priority();
                    one_attack = attack.clone();
                }
                _ => unreachable!(),
            };
            let two_prio;
            let two_attack;
            match self.get_player_two().get_next_move().unwrap() {
                Next::Move(attack) => {
                    two_prio = attack.get_priority();
                    two_attack = attack.clone();
                }
                _ => unreachable!(),
            };
            let mut current = self.get_player_one().get_current();
            let one_speed = self.get_player_one().get_pokemon_list()[current]
                .get_current()
                .get_stat(&enums::Stats::Speed);
            current = self.get_player_two().get_current();
            let two_speed = self.get_player_two().get_pokemon_list()[current]
                .get_current()
                .get_stat(&enums::Stats::Speed);
            // The attack with the higher Priority starts
            //
            if one_prio > two_prio {
                call_resolve(self, one_attack, enums::Player::One, &mut window);
                call_resolve(self, two_attack, enums::Player::Two, &mut window);
            } else if one_prio < two_prio {
                call_resolve(self, two_attack, enums::Player::Two, &mut window);
                call_resolve(self, one_attack, enums::Player::One, &mut window);
            } else {
                // If the attack priority is the same the pokemon with the higher attackspeed starts
                // If the attack speed is the same, the pokemon of player one will strike first
                //
                if one_speed >= two_speed {
                    call_resolve(self, one_attack, enums::Player::One, &mut window);
                    call_resolve(self, two_attack, enums::Player::Two, &mut window);
                } else {
                    call_resolve(self, two_attack, enums::Player::Two, &mut window);
                    call_resolve(self, one_attack, enums::Player::One, &mut window);
                }
            }
        }
        // End of Turn moves like validate the weather and effects, handle poison etc.
        //
        end_of_turn_flags(self, enums::Player::One, window);
        end_of_turn_flags(self, enums::Player::Two, window);
        self.validate_effects_and_weather();
        // TODO: All kind of effect like sleep, paralysis, poison... arent handled yet.
    }
}
/// Resolving if the resolve method must be called and after that if the pokemon is dead
fn call_resolve(arena: &mut super::Arena,
                attack: moves::Technique,
                player: enums::Player,
                mut window: &mut graphic::gui::App) {
    let message_switch;
    // Get the current pokemon
    let current_one = arena.get_player_one().get_current();
    let current_two = arena.get_player_two().get_current();
    // Get the names of the current pokemon
    let message_one = arena.get_player_one().get_pokemon_list()[current_one].get_name();
    let message_two = arena.get_player_two().get_pokemon_list()[current_two].get_name();
    // Checks if the pokemon are dead
    let dead_one = !arena.get_player_one().get_pokemon_list()[current_one].is_alive();
    let dead_two = !arena.get_player_two().get_pokemon_list()[current_two].is_alive();
    // Sets the message_switch for following handles
    match player {
        enums::Player::One => {
            message_switch = message_one.clone();
        }
        enums::Player::Two => {
            message_switch = message_two.clone();
        }
    }

    // Handles confusion and infatuation. If nothing is stops attack, the attack will be resolved
    if confusion(arena, player) {
        match player {
            enums::Player::One => {
                let mut pkmn = arena.get_player_one().get_pokemon_list()[current_one].clone();
                let damage = ((((2.0 * pkmn.get_level() as f32 + 10.0) / 250.0) *
                               pkmn.get_current().get_stat(&enums::Stats::Attack) as f32 /
                               pkmn.get_current().get_stat(&enums::Stats::Defense) as
                               f32 * 40.0 + 2.0)) as u16;
                arena.get_player_one().get_pokemon_list()[current_one]
                    .get_current()
                    .set_stats(enums::Stats::Hp, damage);
            }
            enums::Player::Two => {
                let mut pkmn = arena.get_player_two().get_pokemon_list()[current_two].clone();
                let damage = ((((2.0 * pkmn.get_level() as f32 + 10.0) / 250.0) *
                               pkmn.get_current().get_stat(&enums::Stats::Attack) as f32 /
                               pkmn.get_current().get_stat(&enums::Stats::Defense) as
                               f32 * 40.0 + 2.0)) as u16;
                arena.get_player_two().get_pokemon_list()[current_two]
                    .get_current()
                    .set_stats(enums::Stats::Hp, damage);
            }
        }
        window.set_battle_text(message_switch + " is confused and hitted himself!");
    } else if infatuation(arena, player) {
        window.set_battle_text(message_switch + " has the infatuation effect!");
    } else {
        match player {
            enums::Player::One => {
                if arena.get_player_one().get_next_move().unwrap() == Next::Flinch {
                    window.set_battle_text(message_one.clone() + "flinched.");
                } else {
                    window.set_battle_text(message_one.clone() + " uses " + attack.get_name());
                    attack.resolve(arena, player, &mut window);
                }
            }
            enums::Player::Two => {
                if arena.get_player_two().get_next_move().unwrap() == Next::Flinch {
                    window.set_battle_text(message_one.clone() + " flinched.");
                } else {
                    window.set_battle_text(message_one.clone() + " uses " + attack.get_name());
                    attack.resolve(arena, player, &mut window);
                }
            }
        }
        
    }

    // Swaps the pokemon if its dead
    if dead_one {
        window.set_battle_text(message_one.clone() + "is defeated!");

        let new = window.get_changed_pokemon(player);
        arena.get_player_one().set_current(new);
    }
    if dead_two {
        window.set_battle_text(message_one.clone() + "is defeated!");
        let new = window.get_changed_pokemon(player);
        arena.get_player_two().set_current(new);
    }

}

/// Handles the end of turn flags
fn end_of_turn_flags(arena: &mut super::Arena, player: enums::Player, window: &graphic::gui::App) {
    let current_one = arena.get_player_one().get_current();
    let current_two = arena.get_player_two().get_current();
    let map: HashMap<enums::EndOfTurn, u8> = match player {
        enums::Player::One => {
            arena.get_player_one().get_pokemon_list()[current_one].get_end_of_turn_flags().clone()
        }
        enums::Player::Two => {
            arena.get_player_two().get_pokemon_list()[current_two].get_end_of_turn_flags().clone()
        }
    };
    for i in map.iter() {
        match *i.0 {
            // Absorbs HP of the pokemon
            enums::EndOfTurn::LeechSeed => {
                match player {
                    enums::Player::One => {
                        // Get hp from defending Pokemon
                        let mut hp = arena.get_player_two().get_pokemon_list()[current_two]
                            .get_base()
                            .get_stat(&enums::Stats::Hp);
                        // Get the amount for heal and dmg
                        let absorb = hp / 16;
                        // Damage Defender
                        arena.get_player_two().get_pokemon_list()[current_two]
                            .get_current()
                            .set_stats(enums::Stats::Hp, hp - absorb);
                        // Get HP of attacking Pkmn
                        hp = arena.get_player_one().get_pokemon_list()[current_one]
                            .get_current()
                            .get_stat(&enums::Stats::Hp);
                        // If Atacker isnt fully healed after that action add the absorbed amount to
                        // current Hp
                        if arena.get_player_one().get_pokemon_list()[current_one]
                            .get_base()
                            .get_stat(&enums::Stats::Hp) >=
                           (hp + absorb) {
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp + absorb);
                        } else {
                            // else set the hp to the base value
                            hp = arena.get_player_one().get_pokemon_list()[current_one]
                                .get_base()
                                .get_stat(&enums::Stats::Hp);
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp);
                        }
                        if !arena.get_player_one().get_pokemon_list()[current_one].is_alive() {
                            let new = window.get_changed_pokemon(enums::Player::One);
                            arena.get_player_one().set_current(new);
                        }
                    }
                    enums::Player::Two => {
                        // Get hp from defending Pokemon
                        let mut hp = arena.get_player_one().get_pokemon_list()[current_one]
                            .get_base()
                            .get_stat(&enums::Stats::Hp);
                        // Get the amount for heal and dmg
                        let absorb = hp / 16;
                        // Damage Defender
                        arena.get_player_one().get_pokemon_list()[current_one]
                            .get_current()
                            .set_stats(enums::Stats::Hp, hp - absorb);
                        // Get HP of attacking Pkmn
                        hp = arena.get_player_two().get_pokemon_list()[current_two]
                            .get_current()
                            .get_stat(&enums::Stats::Hp);
                        // If Atacker isnt fully healed after that action add the absorbed amount to
                        // current Hp
                        if arena.get_player_two().get_pokemon_list()[current_two]
                            .get_base()
                            .get_stat(&enums::Stats::Hp) >=
                           (hp + absorb) {
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp + absorb);
                        } else {
                            // else set the hp to the base value
                            hp = arena.get_player_two().get_pokemon_list()[current_two]
                                .get_base()
                                .get_stat(&enums::Stats::Hp);
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp);
                        }
                        if !arena.get_player_two().get_pokemon_list()[current_two].is_alive() {
                            let new = window.get_changed_pokemon(enums::Player::Two);
                            arena.get_player_two().set_current(new);
                        }

                    }
                }
            }
            // After four rounds the pokemon will die
            enums::EndOfTurn::PerishSong => {
                match player {
                    enums::Player::One => {
                        if *i.1 != 4 {
                            *arena.get_player_one().get_pokemon_list()[current_one]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::PerishSong)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .get_current()
                                .set_stats(enums::Stats::Hp, 0);
                            let new = window.get_changed_pokemon(enums::Player::One);
                            arena.get_player_one().set_current(new);
                        }
                    }
                    enums::Player::Two => {
                        if *i.1 > 4 {
                            *arena.get_player_two().get_pokemon_list()[current_two]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::PerishSong)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .get_current()
                                .set_stats(enums::Stats::Hp, 0);
                            let new = window.get_changed_pokemon(enums::Player::Two);
                            arena.get_player_two().set_current(new);
                        }
                    }
                }
            }
            // Pokemon will fall asleep in the next round
            enums::EndOfTurn::Yawn => {
                match player {
                    enums::Player::One => {
                        if *i.1 > 1 {
                            *arena.get_player_one().get_pokemon_list()[current_one]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::Yawn)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .get_end_of_turn_flags()
                                .remove(&enums::EndOfTurn::Yawn);
                            if arena.get_player_one().get_pokemon_list()[current_one]
                                .get_non_volatile()
                                .0 != enums::NonVolatile::Sleep {
                                arena.get_player_one().get_pokemon_list()[current_one]
                                    .set_non_volatile(enums::NonVolatile::Sleep);
                            }
                        }
                    }
                    enums::Player::Two => {
                        if *i.1 > 1 {
                            *arena.get_player_two().get_pokemon_list()[current_two]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::Yawn)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .get_end_of_turn_flags()
                                .remove(&enums::EndOfTurn::Yawn);
                            if arena.get_player_two().get_pokemon_list()[current_two]
                                .get_non_volatile()
                                .0 != enums::NonVolatile::Sleep {
                                arena.get_player_two().get_pokemon_list()[current_two]
                                    .set_non_volatile(enums::NonVolatile::Sleep);
                            }
                        }
                    }
                }
            }
            // Changing the flying type at position one
            enums::EndOfTurn::RoostTypeOne => {
                match player {
                    enums::Player::One => {
                        if *i.1 > 1 {
                            *arena.get_player_one().get_pokemon_list()[current_one]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::RoostTypeOne)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .set_type(0, enums::Types::Flying);
                        }
                    }
                    enums::Player::Two => {
                        if *i.1 > 1 {
                            *arena.get_player_two().get_pokemon_list()[current_two]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::RoostTypeOne)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .set_type(0, enums::Types::Flying);
                        }
                    }

                }
            }
            // Changing the flying type at position two
            enums::EndOfTurn::RoostTypeTwo => {
                match player {
                    enums::Player::One => {
                        if *i.1 > 1 {
                            *arena.get_player_one().get_pokemon_list()[current_one]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::RoostTypeOne)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .set_type(2, enums::Types::Flying);
                        }
                    }
                    enums::Player::Two => {
                        if *i.1 > 1 {
                            *arena.get_player_two().get_pokemon_list()[current_two]
                                .get_end_of_turn_flags()
                                .get_mut(&enums::EndOfTurn::RoostTypeOne)
                                .unwrap() = *i.1 + 1;
                        } else {
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .set_type(2, enums::Types::Flying);
                        }
                    }

                }
            }
            // Pokemon gets damage at the end of each round
            enums::EndOfTurn::Trap => {
                match player {
                    enums::Player::One => {
                        // Get base hp from Pokemon
                        let hp = arena.get_player_one().get_pokemon_list()[current_one]
                            .get_base()
                            .get_stat(&enums::Stats::Hp);
                        // Get the amount for dmg
                        let damage = hp / 8;
                        // Damage pokemon
                        arena.get_player_one().get_pokemon_list()[current_one]
                            .get_current()
                            .set_stats(enums::Stats::Hp, hp - damage);
                        // if pokemon dead force a switch
                        if !arena.get_player_one().get_pokemon_list()[current_one].is_alive() {
                            let new = window.get_changed_pokemon(enums::Player::One);
                            arena.get_player_one().set_current(new);
                        }
                    }
                    enums::Player::Two => {
                        // Get base hp from Pokemon
                        let hp = arena.get_player_two().get_pokemon_list()[current_two]
                            .get_base()
                            .get_stat(&enums::Stats::Hp);
                        // Get the amount for dmg
                        let damage = hp / 8;
                        // Damage pokemon
                        arena.get_player_two().get_pokemon_list()[current_two]
                            .get_current()
                            .set_stats(enums::Stats::Hp, hp - damage);
                        // if pokemon dead force a switch
                        if !arena.get_player_two().get_pokemon_list()[current_two].is_alive() {
                            let new = window.get_changed_pokemon(enums::Player::Two);
                            arena.get_player_two().set_current(new);
                        }
                    }
                }
            }
            // Pokemon get hp at the end of every round
            enums::EndOfTurn::Ingrain => {
                match player {
                    enums::Player::One => {
                        let mut hp = arena.get_player_one().get_pokemon_list()[current_one]
                            .get_base()
                            .get_stat(&enums::Stats::Hp);
                        // Get the amount for heal
                        hp = hp + (hp / 16);
                        if arena.get_player_one().get_pokemon_list()[current_one]
                            .get_base()
                            .get_stat(&enums::Stats::Hp) >= hp {
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp);
                        } else {
                            hp = arena.get_player_one().get_pokemon_list()[current_one]
                                .get_base()
                                .get_stat(&enums::Stats::Hp);
                            arena.get_player_one().get_pokemon_list()[current_one]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp);
                        }
                    }
                    enums::Player::Two => {
                        let mut hp = arena.get_player_two().get_pokemon_list()[current_two]
                            .get_base()
                            .get_stat(&enums::Stats::Hp);
                        // Get the amount for heal
                        hp = hp + (hp / 16);
                        if arena.get_player_two().get_pokemon_list()[current_two]
                            .get_base()
                            .get_stat(&enums::Stats::Hp) >= hp {
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp);
                        } else {
                            hp = arena.get_player_two().get_pokemon_list()[current_two]
                                .get_base()
                                .get_stat(&enums::Stats::Hp);
                            arena.get_player_two().get_pokemon_list()[current_two]
                                .get_current()
                                .set_stats(enums::Stats::Hp, hp);
                        }
                    }
                }

            }
        }
    }
}

/// Handle Confusion
fn confusion(arena: &mut super::Arena, player: enums::Player) -> bool {
    match player {
        enums::Player::One => {
            let current = arena.get_player_one().get_current();
            if arena.get_player_one().get_pokemon_list()[current]
                .get_fight_flags()
                .contains_key(&enums::Fighting::Infatuation) {
                return rand::random::<bool>();
            }
            false
        }
        enums::Player::Two => {
            let current = arena.get_player_two().get_current();
            if arena.get_player_two().get_pokemon_list()[current]
                .get_fight_flags()
                .contains_key(&enums::Fighting::Infatuation) {
                return rand::random::<bool>();
            }
            false
        }
    }

}
/// Handle Infatuation
fn infatuation(arena: &mut super::Arena, player: enums::Player) -> bool {
    match player {
        enums::Player::One => {
            let current = arena.get_player_one().get_current();
            if arena.get_player_one().get_pokemon_list()[current]
                .get_fight_flags()
                .contains_key(&enums::Fighting::Confusion) {
                let random = rand::random::<u8>();
                return random > random / 3;
            }
            false
        }
        enums::Player::Two => {
            let current = arena.get_player_two().get_current();
            if arena.get_player_two().get_pokemon_list()[current]
                .get_fight_flags()
                .contains_key(&enums::Fighting::Confusion) {
                let random = rand::random::<u8>();
                return random > random / 3;
            }
            false
        }
    }

}
