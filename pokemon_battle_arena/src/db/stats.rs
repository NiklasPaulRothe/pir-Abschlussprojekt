use super::pokemon_model::PokemonModel;
use super::determinant_values::Dv;
use super::enums;

///Contains the main stats for every Pokemon.
#[derive(Debug, Clone)]
pub struct Stats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}


impl Stats {
    // pub fn calculate_stats(model: PokemonModel, dv: Dv) -> Stats {
    //     //TODO: Methode erstellen, die die Stats errechnet, das Model wird für die Base stats auf
    //     //jeden Fall gebraucht (evtl ist es einfach nur die Stats zu übergeben) und die DVs auch,
    //     //wenn noch was nötig ist muss das im Kopf ergänzt werden.

    // }

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
}
