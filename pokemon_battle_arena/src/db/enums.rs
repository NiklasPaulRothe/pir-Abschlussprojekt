extern crate num;
extern crate rand;

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

enum_from_primitive! {
    #[derive(Debug, Clone)]
    pub enum Move_Category {
        Damage = 0,
        Ailment = 1,
        Net_good_stats = 2,
        Heal = 3,
        Damage_and_ailment = 4,
        Swagger = 5,
        Damage_And_Lower = 6,
        Damage_And_Raise = 7,
        Damage_And_Heal = 8,
        Ohko = 9,
        Whole_Field_Effect = 10,
        Field_Effect = 11,
        Force_Switch = 12,
        Unique = 13,

    }
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}

enum_from_primitive! {
    #[derive(Debug, Clone)]
    pub enum Stats {
        Undefined = 0,
        Hp = 1,
        Attack = 2,
        Defense = 3,
        Special_Attack = 4,
        Special_Defense = 5,
        Speed = 6
    }
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

pub fn get_gender() -> Gender {
    if rand::random() {
        return Gender::Male
    }
    Gender::Female
}
