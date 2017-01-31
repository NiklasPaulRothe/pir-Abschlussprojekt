extern crate num;
extern crate rand;

use self::num::FromPrimitive;
use self::rand::{Rng, thread_rng};

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
        Net_Good_Stats = 2,
        Heal = 3,
        Damage_And_Ailment = 4,
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
pub enum Ailment {
    Unknown,
    Undefined,
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison,
    Confusion,
    Infatuation,
    Trap,
    Nightmare,
    Torment,
    Disable,
    Yawn,
    Heal_Block,
    No_Type_Immunity,
    Leech_Seed,
    Embargo,
    Perish_Song,
    Ingrain,
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
#[derive(Debug, Clone)]
pub enum Weather {
    Clear_Sky,
    Sunlight,
    //no need to handle it right now, only caused by abilities
    Hars_Sunlight,
    Rain,
    //no need to handle it right now, only caused by abilities
    Heavy_Rain,
    Sandstorm,
    Hail,
    //no need to handle it right now, only caused by abilities
    Air_Current,
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

enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone)]
    pub enum MoveFlags {
        Contact = 1,
        Charge = 2,
        Recharge = 3,
        Protect = 4,
        Reflectable = 5,
        Snatch = 6,
        Mirror = 7,
        Punch = 8,
        Sound = 9,
        Gravity = 10,
        Defrost = 11,
        Distance = 12,
        Heal = 13,
        Authentic = 14,
        Powder = 15,
        Bite = 16,
        Pulse = 17,
        Balistic = 18,
        Mental = 19,
        Non_Sky_Battle = 20,
    }
}

pub fn get_gender(gender_rate: i8) -> Gender {
    let mut rng = thread_rng();
    let probability = rng.gen_range(1, 101);
    match gender_rate {
        -1 => Gender::Genderless,
        0 => Gender::Male,
        1 => {
            if probability < 87 {
            return Gender::Male
            }
        Gender::Female
        },
        2 => {
            if probability < 75 {
            return Gender::Male
            }
        Gender::Female
        },
        4 => {
            if probability < 50 {
            return Gender::Male
            }
        Gender::Female
        },
        6 => {
            if probability < 25 {
            return Gender::Male
            }
        Gender::Female
        },
        7 => {
            if probability < 13 {
            return Gender::Male
            }
        Gender::Female
        },
        8 => Gender::Female,
        _ => Gender::Genderless,
    }
}
