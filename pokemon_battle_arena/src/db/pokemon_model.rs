use super::enums;
use super::stats;

///Basic values for Pokemon species. Equal for every instance of the given Pokemon.
#[derive(Debug, Clone)]
pub struct PokemonModel {
    pub pokedex_id: usize,
    pub name: String,
    pub type_one: enums::types,
    pub type_two: enums::types,
    pub base_stats: stats::Stats,
    pub mega_evolution: Box<Option<PokemonModel>>,
}
