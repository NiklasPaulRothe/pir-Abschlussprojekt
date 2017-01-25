extern crate csv;

use super::pokemon_model;

#[derive(Debug)]
pub struct Nature {
    id: usize,
    name: String,
    decrease_stat: String,
    increase_stat: String,
}

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
