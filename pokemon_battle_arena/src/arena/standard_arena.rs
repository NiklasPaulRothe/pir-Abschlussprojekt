use player::Next;
use db::enums;


/// The standard arena is based on the default 1v1 fight.

impl<'a> super::Arena<'a> {
    /// Simulating a fight. This function is called on a arena and uses the next_move variables of
    /// the players to know what to do in this round.
    /// Important: All next_move variables must contain a Some() entry. If the function is called
    /// and atleast one variable is holding a None, this function will panic!
    pub fn fight(&mut self) {
        // This flag is used to show that the round is "over" earlier as aspected.
        // This can be happen if pursuit was used or both pokemons are swapped.
        let mut end_of_fight = false;
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Setting the switched flag in the Player structs to false and reset if a swap will be done
        ////////////////////////////////////////////////////////////////////////////////////////////
        self.get_player_one().set_switched(false);
        self.get_player_two().set_switched(false);


        ////////////////////////////////////////////////////////////////////////////////////////////
        // Handle the pursuit(ID: 228) attack
        ////////////////////////////////////////////////////////////////////////////////////////////
        match self.get_player_one()
            .get_next_move().0
            .expect("Unexpected error! This field of player one shouldn`t be None at this point.") {
            Next::Move(technique) => {
                if technique.get_id() == 228 {
                    match self.get_player_one()
                        .get_next_move().0
                        .expect("Unexpected error! This field of player one shouldn`t be None \
                                 at this point.") {
                        Next::Switch(_) => {
                            // Resolving pursuit, updating last action and last move
                            // and setting the next move to None
                            technique.resolve(self, 2);
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
                    .get_next_move().0
                    .expect("Unexpected error! This field of player one shouldn`t be None at \
                             this point.") {
                    Next::Move(technique) => {
                        if technique.get_id() == 228 {
                            // Resolving pursuit, updating last action and last move
                            // and setting the next move to None
                            technique.resolve(self, 1);
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Switch Pokemon of Player One if he wants to
        ////////////////////////////////////////////////////////////////////////////////////////////
        if let Some(x) = self.get_player_one().get_next_move().0 {
            match x {
                Next::Switch(pkmn) => {
                    // Switch of the current pokemon + setting flag
                    self.get_player_one().set_current(pkmn.get_int());
                    self.get_player_one().set_switched(true);
                    // Updating last action and setting next move to None. Last Move isnt updated
                    // because the last action wasnt a move
                    let old_move = self.get_player_one().get_next_move().unwrap().clone();
                    self.get_player_one().set_last_action(old_move);
                    self.get_player_one().set_next_move(None);
                }
                _ => {}
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Switch Pokemon of Player Two if he wants to
        ////////////////////////////////////////////////////////////////////////////////////////////
        if let Some(x) = self.get_player_two().get_next_move().0 {
            match x {
                Next::Switch(pkmn) => {
                    // Switch of the current pokemon + setting flag
                    self.get_player_two().set_current(pkmn.get_int());
                    self.get_player_two().set_switched(true);
                    // Updating last action and setting next move to None. Last Move isnt updated
                    // because the last action wasnt a move
                    let old_move = self.get_player_two().get_next_move().unwrap().clone();
                    self.get_player_two().set_last_action(old_move);
                    self.get_player_two().set_next_move(None);
                }
                _ => {}
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // If player one doesnt need to make a move anymore, only resolve attack of player two
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.get_player_one().get_next_move() != Next::None &&
           self.get_player_two().get_next_move() != Next::None {
            match self.get_player_two().get_next_move().unwrap() {
                Next::Move(x, _) => x.resolve(self, 2),
                _ => {}
            }
            end_of_fight = true;
            ////////////////////////////////////////////////////////////////////////////////////////
            // If player two doesnt need to make a move anymore, only resolve attack of player one
            ////////////////////////////////////////////////////////////////////////////////////////
        } else if self.get_player_two().get_next_move() == Next::None &&
                  self.get_player_one().get_next_move() != Next::None {
            match self.get_player_one().get_next_move().unwrap() {
                Next::Move(x) => x.resolve(self, 1),
                _ => {}
            }
            end_of_fight = true;
            ////////////////////////////////////////////////////////////////////////////////////////
            // If both player dont have a move go out of fight
            ////////////////////////////////////////////////////////////////////////////////////////
        } else if self.get_player_two().get_next_move() == Next::None &&
                  self.get_player_one().get_next_move() != Next::None {
            end_of_fight = true;
        }


        ////////////////////////////////////////////////////////////////////////////////////////////
        // If both player want to perform an attack Priority and Speed of Pokemon will be used to
        // decide which pokemon strikes first
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Variables for faster comparison. x_prio is the priority of the pokemon of player x and
        // x_speed is the attackspeed of the pokemon of player x
        if !end_of_fight {
            let one_prio;
            let one_attack;
            match self.get_player_one().get_next_move().unwrap().0 {
                Next::Move(attack) => {
                    one_prio = attack.get_priority();
                    one_attack = attack.clone();
                }
                _ => unreachable!(),
            };
            let two_prio;
            let two_attack;
            match self.get_player_two().get_next_move().unwrap().0 {
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
            ////////////////////////////////////////////////////////////////////////////////////////
            // The attack with the higher Priority starts
            ////////////////////////////////////////////////////////////////////////////////////////
            if one_prio > two_prio {
                one_attack.resolve(self, 1);
                two_attack.resolve(self, 2);
            } else if one_prio < two_prio {
                two_attack.resolve(self, 2);
                one_attack.resolve(self, 1);
            } else {
                ////////////////////////////////////////////////////////////////////////////////////
                // If the attack priority is the same the pokemon with the higher attackspeed starts
                // If the attack speed is the same, the pokemon of player one will strike first
                ////////////////////////////////////////////////////////////////////////////////////
                if one_speed >= two_speed {
                    one_attack.resolve(self, 1);
                    two_attack.resolve(self, 2);
                } else {
                    two_attack.resolve(self, 2);
                    one_attack.resolve(self, 1);
                }
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // End of Turn moves like validate the weather and effects, handle poison etc.
        ////////////////////////////////////////////////////////////////////////////////////////////
        self.validate_effects_and_weather();
        // TODO: All kind of effect like sleep, paralysis, poison... arent handled yet.
    }
}
