use super::moves::Technique;

struct movedex {
    entries: Vec<Technique, TechniqueTmp>,
    complete: bool,
}

impl movedex {
    pub fn move_by_id(&self, id: usize) -> Option<Technique> {
        if id < 622 && self.is_complete() {
            return Some(movedex[id - 1].clone());
        }
        else if id < 622 {
            for entry in self.entries.clone() {
                if entry.attack_id = id {
                    return Some(entry);
                }
            }
        }
        None
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    ///creates similar to the pokedex a Vec that contains all known moves.
    pub fn create_movedex() -> Vec<Technique> {
        let mut effectivity = Vec::new();
        let mut effective_db = csv::Reader::from_file("./src/db/tables/type_efficacy.csv").unwrap();
        for record in effective_db.decode() {
            let(off, def, factor): (i32, i32, u8) = record.unwrap();
            effectivity.push((off, def, factor));
        }
        let mut moves = Vec::new();
        let mut move_db = csv::Reader::from_file("./src/db/tables/moves.csv").unwrap();
        for record in move_db.decode() {
            let tmp: TechniqueTmp = record.unwrap();
            let chance = match tmp.effect_chance {
                Some(n) => n,
                None => 100,
            };
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
                let attack = Technique {
                    attack_id: tmp.attack_id,
                    name: tmp.name,
                    attack_type: enums::types::from_i32(tmp.attack_type).
                    unwrap_or(enums::types::undefined),
                    power: tmp.power,
                    power_points: tmp.power_points,
                    accuracy: tmp.accuracy,
                    has_priority: { tmp.has_priority == Some(1) },
                    target: enums::Target::from_i32(tmp.target).unwrap(),
                    typeeffectiveness: effective_hash,
                    damage_class: enums::DamageClass::from_i32(tmp.damage_class).unwrap(),
                    effect_id: tmp.effect,
                    effect_chance: chance,
                };
                moves.push(attack);
            }
        }

        let mut effective_db = csv::Reader::from_file("./src/db/tables/type_efficacy.csv").unwrap();
        for record in effective_db.decode() {
            let(off, def, factor): (i32, i32, u8) = record.unwrap();

        }
        Movedex {
            entries:moves,
            complete: true,
        }
    }
}
