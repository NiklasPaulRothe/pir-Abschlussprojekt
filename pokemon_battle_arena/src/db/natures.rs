extern crate csv;
extern crate rand;

use super::pokemon_model;
use self::rand::{thread_rng, sample};

#[derive(Debug, Clone)]
pub struct Nature {
    id: usize,
    name: String,
    decrease_stat: String,
    increase_stat: String,
}

///creates a Vec with all natures in it.
pub fn create_naturedex() -> Vec<Nature> {
    let mut natures = Vec::new();
    let mut nature_db = csv::Reader::from_file("./src/db/tables/natures.csv").unwrap();
    for record in nature_db.decode() {
        let (id, name, decrease, increase, _, _, _): (usize, String, u8, u8, usize, usize, usize) =
        record.unwrap();
        natures.push( Nature {
            id: id,
            name: name,
            decrease_stat: {
                match decrease {
                    1 => String::from("hp"),
                    2 => String::from("attack"),
                    3 => String::from("defense"),
                    4 => String::from("special-attack"),
                    5 => String::from("special-defense"),
                    6 => String::from("speed"),
                    _ => unreachable!(),
                }
            },
            increase_stat: {
                match increase {
                    1 => String::from("hp"),
                    2 => String::from("attack"),
                    3 => String::from("defense"),
                    4 => String::from("special-attack"),
                    5 => String::from("special-defense"),
                    6 => String::from("speed"),
                    _ => unreachable!(),
                }
            },
        })
    }
    natures
}

pub fn get_random_nature() -> Nature {
    let dex = create_naturedex();
    let mut rng = thread_rng();
    let nature = sample(&mut rng, dex.iter(), 1);
    nature[0].clone()
}
