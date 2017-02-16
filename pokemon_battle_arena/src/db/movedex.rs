extern crate csv;

use super::enums;
use super::moves::Technique;
use enum_primitive::FromPrimitive;
use std::collections::HashMap;



/// Manages the list of moves that are available. Contains a bool that is true whenever all
/// available moves are inside the entries to make an easier search possible.
/// By now the whole movedex contains 617 moves, which are nearly all moves from the main game
/// series. 4 Moves are missing due to missing data in the used database.
#[derive(Debug, Clone)]
pub struct Movedex {
    entries: Vec<Technique>,
    complete: bool,
}

// TODO: last 4 attacks are missing in move_meta.csv, therefore are not implemented right now.
// DB must be extended and if statements adjusted accordingly

impl Movedex {
    /// Takes an ID and a movedex and returns an option with the move that can be find with the
    /// given ID. Returns None if the ID isn't in the movedex.
    pub fn move_by_id(&self, id: usize) -> Option<Technique> {
        if id < 617 && self.is_complete() {
            return Some(self.get_entries()[id - 1].clone());
        } else if id < 617 {
            for entry in self.entries.clone() {
                if entry.get_id() == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    /// Returns a list of all learnable moves by level for a specific pokemon with a specific
    /// level.
    pub fn for_token(&self, level: u16, id: usize) -> Movedex {
        let mut new_dex = Vec::new();
        let mut move_db = csv::Reader::from_file("./src/db/tables/pokemon_moves.csv").unwrap();
        for record in move_db.decode() {
            let (poke_id, version, move_id, _, move_level, _): (usize,
                                                                u8,
                                                                usize,
                                                                usize,
                                                                u16,
                                                                Option<usize>) = record.unwrap();
            if move_id < 617 && move_level <= level && poke_id == id && version == 16 {
                let move_tmp = self.move_by_id(move_id);
                // ifs are needed to exclude unimplemented moves from the list
                if move_tmp.clone().is_some() &&
                   !(move_tmp.clone().unwrap().get_name() == "counter" ||
                     move_tmp.clone().unwrap().get_name() == "bide" ||
                     move_tmp.clone().unwrap().get_name() == "mirror-coat" ||
                     move_tmp.clone().unwrap().get_name() == "spit-up" ||
                     move_tmp.clone().unwrap().get_name() == "natural-gift" ||
                     move_tmp.clone().unwrap().get_name() == "metal-burst" ||
                     move_tmp.clone().unwrap().get_name() == "fling" ||
                     move_tmp.clone().unwrap().get_name() == "trump-card" ||
                     move_tmp.clone().unwrap().get_name() == "me-first" ||
                     move_tmp.unwrap().get_category() == enums::MoveCategory::Unique) ||
                   (move_tmp.unwrap().get_category() == enums::MoveCategory::Unique &&
                    (move_tmp.clone().unwrap().get_name() == "teleport" ||
                     move_tmp.clone().unwrap().get_name() == "mimic" ||
                     move_tmp.clone().unwrap().get_name() == "metronome" ||
                     move_tmp.clone().unwrap().get_name() == "mirror-move" ||
                     move_tmp.clone().unwrap().get_name() == "nature-power" ||
                     move_tmp.clone().unwrap().get_name() == "splash" ||
                     //move_tmp.clone().unwrap().get_name() == "rest" ||
                     move_tmp.clone().unwrap().get_name() == "conversion" ||
                     move_tmp.clone().unwrap().get_name() == "spite" ||
                     move_tmp.clone().unwrap().get_name() == "sleep-talk" ||
                     move_tmp.clone().unwrap().get_name() == "celebrate" ||
                     move_tmp.clone().unwrap().get_name() == "powder" ||
                     move_tmp.clone().unwrap().get_name() == "reflect-type" ||
                     move_tmp.clone().unwrap().get_name() == "soak" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "" ||
                     move_tmp.clone().unwrap().get_name() == "")) {
                    new_dex.push(self.move_by_id(move_id).unwrap());

                }
            }
        }
        new_dex.sort();
        new_dex.dedup();
        Movedex {
            entries: new_dex,
            complete: false,
        }
    }

    /// Returns the entry field of a movedex.
    pub fn get_entries(&self) -> Vec<Technique> {
        self.entries.clone()
    }

    /// Returns true if the movedex contains all possible moves, and false if not.
    fn is_complete(&self) -> bool {
        self.complete
    }

    /// Creates a complete Movedex from the type_efficacy, moves_whole and move_flag_map databases
    /// in the table folder.
    pub fn new() -> Movedex {
        // In the first step creates a vec with the effectivities for every type.
        let mut effectivity = Vec::new();
        let mut effective_db = csv::Reader::from_file("./src/db/tables/type_efficacy.csv").unwrap();
        for record in effective_db.decode() {
            let (off, def, factor): (i32, i32, u8) = record.unwrap();
            effectivity.push((off, def, factor));
        }

        // Creates the main part with most simpel values and directly adds a Hash Map for the type
        // efficiency of the move.
        let mut moves = Vec::new();
        let mut move_db = csv::Reader::from_file("./src/db/tables/moves_whole.csv").unwrap();
        for record in move_db.decode() {
            let mut move_tmp: Technique = record.unwrap();
            let mut effective_hash = HashMap::new();
            for entry in effectivity.clone() {
                if entry.0 == move_tmp.get_type() as i32 && entry.2 != 100 {
                    let eff_id = match entry.2 {
                        0 => -4,
                        50 => -1,
                        200 => 1,
                        _ => unreachable!(),
                    };
                    effective_hash.insert(enums::Types::from_i32(entry.1).unwrap(), eff_id);
                    move_tmp.set_effectivity_map(effective_hash.clone());
                }

            }

            moves.push(move_tmp);
        }

        // Adds all flags, that are valid for the moves.
        let mut flags = Vec::new();
        let mut last_id = 1;
        let mut flag_db = csv::Reader::from_file("./src/db/tables/move_flag_map.csv").unwrap();
        for record in flag_db.decode() {
            let (id, identifier): (usize, i32) = record.unwrap();
            if id < 617 {
                if !(id == last_id) {
                    moves[last_id - 1].set_flags(flags);
                    last_id = id;
                    flags = Vec::new();
                }
                flags.push(enums::MoveFlags::from_i32(identifier).unwrap());
            }
        }

        Movedex {
            entries: moves,
            complete: true,
        }
    }
}
