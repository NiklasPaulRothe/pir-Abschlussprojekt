extern crate num;

use self::num::FromPrimitive;

///enum for the pokemon/attack types.
///Can be assigned from i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone, Eq, PartialEq, Hash)]
    pub enum types {
        normal = 1,
        fighting = 2,
        flying = 3,
        poison = 4,
        ground = 5,
        rock = 6,
        bug = 7,
        ghost = 8,
        steel = 9,
        fire = 10,
        water = 11,
        grass = 12,
        electric = 13,
        psychic = 14,
        ice = 15,
        dragon = 16,
        dark = 17,
        fairy = 18,
        undefined = 19,
    }
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}

///enum for the Damage Class of a attack.
///Can be assigned from a i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone)]
    pub enum DamageClass {
        Physical = 1,
        Special = 2,
        Status = 3,
    }
}

#[derive(Debug)]
pub enum TypeEffectiveness {
    Ineffective,
    NotEffective,
    NotVeryEffective,
    Normal,
    VeryEffective,
    SuperEffective,
}

///Enum that contains the valid target(s) of a move.
///Can be assigned from a i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone)]
    pub enum Target {
        SpecificMove = 1,
        SelectedPokemonMeFirst = 2,
        Ally = 3,
        UsersField = 4,
        UserOrAlly = 5,
        OpponentsField = 6,
        User = 7,
        RandomOpponent = 8,
        AllOtherPokemon = 9,
        SelectedPokemon = 10,
        AllOpponents = 11,
        EntireField = 12,
        UserAndAllies = 13,
        AllPokemon = 14,
    }
}
