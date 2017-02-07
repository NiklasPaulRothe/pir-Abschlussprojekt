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
    pub fn calculate_stats(model: PokemonModel, dv: Dv) -> Stats {
        let mut level = 50;
        let hp = (
            (2 * model.base_stats + dv.hp * level) / 100.0 + level + 10
            ) as u16;

        fn stat_formula(model: PokemonModel, level: u8) -> u16 {
            ((2 * model + dv.hp * level) / 100.0 + 5.0) as u16
        }

        Stats {
            hp: hp,
            attack: stat_formula(model.attack, level),
            defense: stat_formula(model.defense, level),
            speed: stat_formula(model.speed, level),
            special_attack: stat_formula(model.special_attack, level),
            special_defense: stat_formula(model.special_defense, level),
        }
    }

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
