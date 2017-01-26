extern crate csv;

use super::moves::{Technique, TechniqueTmp};
use super::enums;
use std::collections::HashMap;
use enum_primitive::FromPrimitive;

pub struct Movedex {
    entries: Vec<Technique>,
    complete: bool,
}

impl Movedex {
    pub fn move_by_id(&self, id: usize) -> Option<Technique> {
        if id < 622 && self.is_complete() {
            return Some(self.get_entries()[id - 1].clone());
        }
        else if id < 622 {
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
        let mut descriptions = Vec::new();
        let mut flavor_db = csv::Reader::from_file("./src/db/tables/move_flavor_text.csv").unwrap();
        for record in flavor_db.decode() {
            let (attack_id, version, language, description):(usize, u8, u8, String)
            = record.unwrap();
            if attack_id < 622 && version == 16 && language == 9 {
                let mut desc = String::new();
                for elem in description.split_whitespace() {
                    desc.push_str(elem);
                    desc.push_str(" ");
                }
                desc.trim();
                descriptions.push(desc);
            }
        }
        let mut moves = Vec::new();
        let mut move_db = csv::Reader::from_file("./src/db/tables/moves.csv").unwrap();
        for record in move_db.decode() {
            let tmp: TechniqueTmp = record.unwrap();
            let mut effective_hash = HashMap::new();
            for entry in effectivity.clone() {
                if entry.0 == tmp.attack_type as i32 && entry.2 != 100 {
                    let eff_id = match entry.2 {
                        0 => -4,
                        50 => -1,
                        200 => 1,
                        _ => unreachable!(),
                    };
                    effective_hash.insert(enums::types::from_i32(entry.1).unwrap(), eff_id);
                }
            }
            if tmp.attack_id < 622 {
                let attack = Technique::from_tmp(tmp.clone(), effective_hash,
                    descriptions[tmp.attack_id - 1].clone());
                moves.push(attack);
            }
        }
        Movedex {
            entries:moves,
            complete: true,
        }
    }
}
