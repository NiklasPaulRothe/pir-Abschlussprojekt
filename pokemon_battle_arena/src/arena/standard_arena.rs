use db::{enums, moves};
use db::moves::{get_attacker, get_target, get_user};
use graphic;
use player::Next;
use rand;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

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
        self.get_player_one().set_switched(false);
        self.get_player_two().set_switched(false);

        // Handle the pursuit(ID: 228) attack
        // If one pokemon wants to swap and the other pokemon is using pursuit. The attack will be
        // handle before swapping the pokemon
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
                            call_resolve(self, technique, enums::Player::One, &mut window);
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
                            call_resolve(self, technique, enums::Player::Two, &mut window);
                            self.get_player_two().set_next_move(None);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        // Switch Pokemon of Player One if he wants to
        if let Some(x) = self.get_player_one().get_next_move() {
            match x {
                Next::Switch(pkmn) => {
                    // Switch of the current pokemon + setting flag
                    self.get_player_one().set_current(pkmn.get_int() - 1);
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
        if let Some(x) = self.get_player_two().get_next_move() {
            match x {
                Next::Switch(pkmn) => {
                    // Switch of the current pokemon + setting flag
                    self.get_player_two().set_current(pkmn.get_int() - 1);
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
        if self.get_player_one().get_next_move().is_none() &&
           self.get_player_two().get_next_move().is_some() {
            match self.get_player_two().get_next_move().unwrap() {
                Next::Move(x) => {
                    call_resolve(self, x, enums::Player::Two, &mut window);
                }
                _ => {}
            }
            end_of_fight = true;
            // If player two doesnt need to make a move anymore, only resolve attack of player one
        } else if self.get_player_two().get_next_move().is_none() &&
                  self.get_player_one().get_next_move().is_some() {
            match self.get_player_one().get_next_move().unwrap() {
                Next::Move(x) => {
                    call_resolve(self, x, enums::Player::One, &mut window);
                }
                _ => {}
            }
            end_of_fight = true;
            // If both player dont have a move go out of fight
        } else if self.get_player_two().get_next_move().is_none() &&
                  self.get_player_one().get_next_move().is_none() {
            end_of_fight = true;
        }


        // If both player want to perform an attack Priority and Speed of Pokemon will be used to
        // decide which pokemon strikes first

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
            let mut one_speed = self.get_player_one().get_pokemon_list()[current]
                .get_current()
                .get_stat(&enums::Stats::Speed);
            // If the pokemon one is paralysed reduce speed by 50%
            if self.get_player_one().get_pokemon_list()[current].get_non_volatile().0 ==
               enums::NonVolatile::Paralysis {
                one_speed /= 2;
            }
            current = self.get_player_two().get_current();
            let mut two_speed = self.get_player_two().get_pokemon_list()[current]
                .get_current()
                .get_stat(&enums::Stats::Speed);
            // If the pokemon two is paralysed reduce speed by 50%
            if self.get_player_two().get_pokemon_list()[current].get_non_volatile().0 ==
               enums::NonVolatile::Paralysis {
                two_speed /= 2;
            }
            // The attack with the higher Priority starts
            if one_prio > two_prio {
                if !call_resolve(self, one_attack, enums::Player::Two, &mut window) {
                    call_resolve(self, two_attack, enums::Player::One, &mut window);
                }
            } else if one_prio < two_prio {
                if !call_resolve(self, two_attack, enums::Player::One, &mut window) {
                    call_resolve(self, one_attack, enums::Player::Two, &mut window);
                }
            } else {
                // If the attack priority is the same the pokemon with the higher attackspeed starts
                // If the attack speed is the same, the pokemon of player one will strike first
                if one_speed >= two_speed {
                    if !call_resolve(self, one_attack, enums::Player::Two, &mut window) {
                        call_resolve(self, two_attack, enums::Player::One, &mut window);
                    }
                } else {
                    if !call_resolve(self, two_attack, enums::Player::One, &mut window) {
                        call_resolve(self, one_attack, enums::Player::Two, &mut window);
                    }
                }
            }
        }

        // Handles Poison and Burn aswell as BadPoison(Bad Poison is dealing the same damage as
        // Poison right now)
        // Player One
        let mut non_volatile = get_target(enums::Player::One, self).get_non_volatile().0;
        if (non_volatile == enums::NonVolatile::Poison) ||
           (non_volatile == enums::NonVolatile::Burn) ||
           (non_volatile == enums::NonVolatile::BadPoison) {
            window.set_battle_text(get_target(enums::Player::One, self).get_name().clone() +
                                   " got damage by " +
                                   non_volatile.to_string().as_str());
            poison_burn_damage(self, enums::Player::One);
        }
        // Player Two
        non_volatile = get_target(enums::Player::Two, self).get_non_volatile().0;
        if (non_volatile == enums::NonVolatile::Poison) ||
           (non_volatile == enums::NonVolatile::Burn) ||
           (non_volatile == enums::NonVolatile::BadPoison) {
            window.set_battle_text(get_target(enums::Player::Two, self).get_name().clone() +
                                   " got damage by " +
                                   non_volatile.to_string().as_str());
            poison_burn_damage(self, enums::Player::Two);
        }

        // Checks if one of the two player died
        check_dead(enums::Player::One, self, window);
        check_dead(enums::Player::Two, self, window);
    }
}
/// Resolving if the resolve method must be called and after that if the pokemon is dead
fn call_resolve(arena: &mut super::Arena,
                attack: moves::Technique,
                player: enums::Player,
                mut window: &mut graphic::gui::App)
                -> bool {
    let mut attack_is_allowed = true;
    // Checks if the pokemon is allowed to attack. This is influeced by Sleep, Freeze and Paralysis
    // Checks if the pokemon is paralysed
    if get_target(player, arena).get_non_volatile().0 == enums::NonVolatile::Paralysis {
        let mut rng = thread_rng();
        // With a chance of 25% the pokemon will not attack
        if rng.gen_range(0, 4) != 0 {
            attack_is_allowed = false;
            window.set_battle_text(get_target(player, arena).get_name().clone() + " is paralysed!");
        }
        // Checks it the pokemon is sleeping
    } else if get_target(player, arena).get_non_volatile().0 == enums::NonVolatile::Sleep {
        // If the pokemon is not sleeping for atleast one round it will not wake up
        if get_target(player, arena).get_non_volatile().1 == 0 {
            window.set_battle_text(get_target(player, arena).get_name().clone() + " is sleeping!");
            attack_is_allowed = false;
            // Pokemon will wake up after 3 rounds
        } else if get_target(player, arena).get_non_volatile().1 >= 3 {
            window.set_battle_text(get_target(player, arena).get_name().clone() + " wakes up.");
            get_target(player, arena).set_non_volatile(enums::NonVolatile::Undefined);
        } else {
            // Check if pokemon will wake up
            if rand::random::<bool>() {
                window.set_battle_text(get_target(player, arena).get_name().clone() + " wakes up.");
                get_target(player, arena).set_non_volatile(enums::NonVolatile::Undefined);
            } else {
                window.set_battle_text(get_target(player, arena).get_name().clone() +
                                       " is sleeping!");
                attack_is_allowed = false;
            }
        }
    } else if get_target(player, arena).get_non_volatile().0 == enums::NonVolatile::Freeze {
        let mut rng = thread_rng();
        // With a chance of 20% the pokemon will not attack
        if rng.gen_range(0, 5) != 0 {
            attack_is_allowed = false;
            window.set_battle_text(get_target(player, arena).get_name().clone() + " is frozen!");
        } else {
            window.set_battle_text(get_target(player, arena).get_name().clone() +
                                   " is not frozen anymore!");
            get_target(player, arena).set_non_volatile(enums::NonVolatile::Undefined);
        }
    }

    if attack_is_allowed {
        // Get the names of the current pokemon
        let message = get_user(player, arena).get_name().clone();
        // Handles confusion and infatuation. If nothing is stops attack, the attack will be
        // resolved
        if confusion(arena, player) {
            let mut pkmn = get_target(player, arena).clone();
            let damage = ((((2.0 * pkmn.get_level() as f32 + 10.0) / 250.0) *
                           pkmn.get_current().get_stat(&enums::Stats::Attack) as f32 /
                           pkmn.get_current().get_stat(&enums::Stats::Defense) as f32 *
                           40.0 + 2.0)) as u16;
            get_target(player, arena).get_current().set_stats(enums::Stats::Hp, damage);
            window.set_battle_text(message + " is confused and hitted himself!");
        } else if infatuation(arena, player) {
            window.set_battle_text(message + " has the infatuation effect!");
        } else {
            if get_attacker(player, arena).get_next_move().unwrap() == Next::Flinch {
                window.set_battle_text(message.clone() + " flinched.");
                let last = get_attacker(player, arena).get_next_move().unwrap();
                get_attacker(player, arena).set_last_action((last, 0));
                get_attacker(player, arena).set_next_move(None);
            } else {
                window.set_battle_text(message.clone() + " uses " + attack.get_name());
                attack.resolve(arena, player, &mut window);
            }
        }
    }
    // Checks if one of the two player died
    match player {
        enums::Player::One => {
            if check_dead(enums::Player::One, arena, window) {
                check_dead(enums::Player::Two, arena, window);
                return true;
            }
            false
        }
        enums::Player::Two => {
            if check_dead(enums::Player::Two, arena, window) {
                check_dead(enums::Player::One, arena, window);
                return true;
            }
            false
        }
    }

}

/// Handles the end of turn flags
fn end_of_turn_flags(arena: &mut super::Arena,
                     player: enums::Player,
                     mut window: &mut graphic::gui::App) {
    let map: HashMap<enums::EndOfTurn, u8> =
        get_target(player, arena).get_end_of_turn_flags().clone();

    for i in map.iter() {
        match *i.0 {
            // Absorbs HP of the pokemon
            enums::EndOfTurn::LeechSeed => {
                // Get hp from defending Pokemon
                let mut hp = get_user(player, arena)
                    .get_base()
                    .get_stat(&enums::Stats::Hp);
                // Get the amount for heal and dmg
                let absorb = hp / 16;
                // Damage Defender
                get_user(player, arena)
                    .get_current()
                    .set_stats(enums::Stats::Hp, hp - absorb);
                // Get HP of attacking Pkmn
                hp = get_target(player, arena)
                    .get_current()
                    .get_stat(&enums::Stats::Hp);
                // If Atacker isnt fully healed after that action add the absorbed amount to
                // current Hp
                if get_target(player, arena)
                    .get_base()
                    .get_stat(&enums::Stats::Hp) >= (hp + absorb) {
                    get_target(player, arena)
                        .get_current()
                        .set_stats(enums::Stats::Hp, hp + absorb);
                } else {
                    // else set the hp to the base value
                    hp = get_target(player, arena)
                        .get_base()
                        .get_stat(&enums::Stats::Hp);
                    get_target(player, arena)
                        .get_current()
                        .set_stats(enums::Stats::Hp, hp);
                }
                if !get_target(player, arena).is_alive() {
                    window.set_screen(graphic::gui::Screen::Switch);
                }
            }
            // After four rounds the pokemon will die
            enums::EndOfTurn::PerishSong => {
                if *i.1 != 4 {
                    *get_target(player, arena)
                        .get_end_of_turn_flags()
                        .get_mut(&enums::EndOfTurn::PerishSong)
                        .unwrap() = *i.1 + 1;
                } else {
                    get_target(player, arena)
                        .get_current()
                        .set_stats(enums::Stats::Hp, 0);
                    window.set_screen(graphic::gui::Screen::Switch);
                }
            }
            // Pokemon will fall asleep in the next round
            enums::EndOfTurn::Yawn => {
                if *i.1 > 1 {
                    *get_target(player, arena)
                        .get_end_of_turn_flags()
                        .get_mut(&enums::EndOfTurn::Yawn)
                        .unwrap() = *i.1 + 1;
                } else {
                    get_target(player, arena)
                        .get_end_of_turn_flags()
                        .remove(&enums::EndOfTurn::Yawn);
                    if get_target(player, arena)
                        .get_non_volatile()
                        .0 != enums::NonVolatile::Sleep {
                        get_target(player, arena).set_non_volatile(enums::NonVolatile::Sleep);
                    }
                }
            }
            // Changing the flying type at position one
            enums::EndOfTurn::RoostTypeOne => {
                if *i.1 > 1 {
                    *get_target(player, arena)
                        .get_end_of_turn_flags()
                        .get_mut(&enums::EndOfTurn::RoostTypeOne)
                        .unwrap() = *i.1 + 1;
                } else {
                    get_target(player, arena).set_type(0, enums::Types::Flying);
                }
            }
            // Changing the flying type at position two
            enums::EndOfTurn::RoostTypeTwo => {
                if *i.1 > 1 {
                    *get_target(player, arena)
                        .get_end_of_turn_flags()
                        .get_mut(&enums::EndOfTurn::RoostTypeOne)
                        .unwrap() = *i.1 + 1;
                } else {
                    get_target(player, arena).set_type(2, enums::Types::Flying);
                }
            }
            // Pokemon gets damage at the end of each round
            enums::EndOfTurn::Trap => {
                // Get base hp from Pokemon
                let hp = get_target(player, arena)
                    .get_base()
                    .get_stat(&enums::Stats::Hp);
                // Get the amount for dmg
                let damage = hp / 8;
                // Damage pokemon
                get_target(player, arena)
                    .get_current()
                    .set_stats(enums::Stats::Hp, hp - damage);
                // if pokemon dead force a switch
                if !get_target(player, arena).is_alive() {
                    window.set_screen(graphic::gui::Screen::Switch);
                }

            }
            // Pokemon get hp at the end of every round
            enums::EndOfTurn::Ingrain => {
                let mut hp = get_target(player, arena)
                    .get_base()
                    .get_stat(&enums::Stats::Hp);
                // Get the amount for heal
                hp = hp + (hp / 16);
                if get_target(player, arena)
                    .get_base()
                    .get_stat(&enums::Stats::Hp) >= hp {
                    get_target(player, arena)
                        .get_current()
                        .set_stats(enums::Stats::Hp, hp);
                } else {
                    hp = get_target(player, arena)
                        .get_base()
                        .get_stat(&enums::Stats::Hp);
                    get_target(player, arena)
                        .get_current()
                        .set_stats(enums::Stats::Hp, hp);
                }
            }
        }
    }
}

/// Handle Confusion
fn confusion(arena: &mut super::Arena, player: enums::Player) -> bool {
    if get_target(player, arena)
        .get_fight_flags()
        .contains_key(&enums::Fighting::Infatuation) {
        return rand::random::<bool>();
    }
    false
}
/// Handle Infatuation
fn infatuation(arena: &mut super::Arena, player: enums::Player) -> bool {
    if get_target(player, arena)
        .get_fight_flags()
        .contains_key(&enums::Fighting::Confusion) {
        let random = rand::random::<u8>();
        return random > random / 3;
    }
    false
}
/// Deals the burn and poison damage.
fn poison_burn_damage(arena: &mut super::Arena, player: enums::Player) {
    let base_hp = get_target(player, arena)
        .get_base()
        .get_stat(&enums::Stats::Hp);
    // Get the amount for heal
    let damage = base_hp / 8;
    let current_hp = get_target(player, arena).get_current().get_stat(&enums::Stats::Hp);
    if !(damage > current_hp) {
        get_target(player, arena).get_current().set_stats(enums::Stats::Hp, current_hp - damage);
    } else {
        get_target(player, arena).get_current().set_stats(enums::Stats::Hp, 0);
    }
}

/// Checks if the pokemon are dead
fn check_dead(player: enums::Player,
              arena: &mut super::Arena,
              mut window: &mut graphic::gui::App)
              -> bool {
    // Checks if the pokemon are dead
    let dead = !get_target(player, arena).is_alive();
    let message = get_target(player, arena).get_name().clone();
    // Swaps the pokemon if its dead
    if dead {
        window.set_battle_text(message.clone() + " is defeated!");
        window.set_screen(graphic::gui::Screen::Switch);
        return true;
    }
    false
}
