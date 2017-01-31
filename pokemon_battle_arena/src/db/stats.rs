use super::pokemon_model::PokemonModel;
use super::determinant_values::Dv;
use super::enums;

///Contains the main stats for every Pokemon.
#[derive(Debug, Clone)]
pub struct Stats {
    hp: u16,
    attack: u16,
    defense: u16,
    special_attack: u16,
    special_defense: u16,
    speed: u16,
}


impl Stats {
    // pub fn calculate_stats(model: PokemonModel, dv: Dv) -> Stats {
    //     //TODO: Methode erstellen, die die Stats errechnet, das Model wird für die Base stats auf
    //     //jeden Fall gebraucht (evtl ist es einfach nur die Stats zu übergeben) und die DVs auch,
    //     //wenn noch was nötig ist muss das im Kopf ergänzt werden.

    // }

    pub fn get_stat(&self, stat: enums::Stats) -> u16 {
        match stat {
            enums::Stats::Hp => self.hp,
            enums::Stats::Attack => self.attack,
            enums::Stats::Defense => self.defense,
            enums::Stats::Special_Attack => self.special_attack,
            enums::Stats::Special_Defense => self.special_defense,
            enums::Stats::Speed => self.speed,
            _=> 0,
        }
    }

    pub fn set_stats(&mut self, stat: enums::Stats, value: u16) {
        match stat {
            enums::Stats::Hp => self.hp = value,
            enums::Stats::Attack => self.attack = value,
            enums::Stats::Defense => self.defense = value,
            enums::Stats::Special_Attack => self.special_attack = value,
            enums::Stats::Special_Defense => self.special_defense = value,
            enums::Stats::Speed => self.speed = value,
            _=> {},
        }
    }

    pub fn default() -> Stats {
        Stats {
            hp: 0,
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
        }
    }
}
