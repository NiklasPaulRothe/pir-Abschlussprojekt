extern crate csv;

use super::moves::{Technique};
use super::enums;
use std::collections::HashMap;
use enum_primitive::FromPrimitive;

pub struct Movedex {
    entries: Vec<Technique>,
    complete: bool,
}

//TODO: last 4 attacks are missing in move_meta.csv, therefore are not implemented right now.
//DB must be extended and if statements adjusted accordingly

impl Movedex {
    pub fn move_by_id(&self, id: usize) -> Option<Technique> {
        if id < 617 && self.is_complete() {
            return Some(self.get_entries()[id - 1].clone());
        }
        else if id < 617 {
            for entry in self.entries.clone() {
                if entry.get_id() == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    pub fn get_entries(&self) -> Vec<Technique> {
        self.entries.clone()
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    ///creates similar to the pokedex a Vec that contains all known moves.
    pub fn new() -> Movedex {
        let mut effectivity = Vec::new();
        let mut effective_db = csv::Reader::from_file("./src/db/tables/type_efficacy.csv").unwrap();
        for record in effective_db.decode() {
            let(off, def, factor): (i32, i32, u8) = record.unwrap();
            effectivity.push((off, def, factor));
        }

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
                    effective_hash.insert(enums::types::from_i32(entry.1).unwrap(), eff_id);
                    move_tmp.set_effectivity_map(effective_hash.clone());
                }

            }

            moves.push(move_tmp);
        }

        let mut flags = Vec::new();
        let mut last_id = 1;
        let mut flag_db = csv::Reader::from_file("./src/db/tables/move_flag_map.csv").unwrap();
        for record in flag_db.decode() {
            let (id, identifier): (usize, i32) = record.unwrap();
            if id < 617 {
                if !(id == last_id) {
                    moves[last_id -1].set_flags(flags);
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
