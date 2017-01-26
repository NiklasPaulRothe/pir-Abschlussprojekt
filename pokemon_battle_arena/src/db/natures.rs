extern crate csv;
extern crate rand;

use super::pokemon_model;
use super::enums;
use self::rand::{thread_rng, sample};
use enum_primitive::FromPrimitive;

#[derive(Debug, Clone)]
pub struct Nature {
    id: usize,
    name: String,
    decrease_stat: enums::Stats,
    increase_stat: enums::Stats,
}

///creates a Vec with all natures in it.
pub fn create_naturedb() -> Vec<Nature> {
    let mut natures = Vec::new();
    let mut nature_db = csv::Reader::from_file("./src/db/tables/natures.csv").unwrap();
    for record in nature_db.decode() {
        let (id, name, decrease, increase, _, _, _): (usize, String, i32, i32, usize, usize, usize) =
        record.unwrap();
        natures.push( Nature {
            id: id,
            name: name,
            decrease_stat: enums::Stats::from_i32(decrease).unwrap(),
            increase_stat: enums::Stats::from_i32(increase).unwrap(),
        })
    }
    natures
}

impl Nature {
    pub fn get_random_nature() -> Nature {
        let dex = create_naturedb();
        let mut rng = thread_rng();
        let nature = sample(&mut rng, dex.iter(), 1);
        nature[0].clone()
    }

    pub fn get_stats(&self) -> (enums::Stats, enums::Stats) {
        (self.decrease_stat.clone(), self.increase_stat.clone())
    }
}
