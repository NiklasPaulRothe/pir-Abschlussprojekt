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

///Enum for the Categories a move can have. They are used to get smaller samples of moves when
///resolve their effects.
enum_from_primitive! {
    #[derive(Debug, Clone, PartialEq)]
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

///All ailments that are known and can be caused by one or more moves.
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

///All the major status Changes that can not be caused at the same time.
#[derive(Debug, Clone, PartialEq)]
pub enum Non_Volatile {
    Undefined,
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison,
    Bad_Poison,
}

///Flags that have a influence at the end of each turn.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum End_Of_Turn {
    //absorbs some HP at the End of every Turn
    Leech_Seed,
    //Counts from 0 to 4, one step each round, even in the turn it was initially used.
    //When 4 is reached the Pokemon faints. Counting only continues when the Pokemon is part of
    //the battle, but the counter will not be resetted if the Pokemon is changed.
    Perish_Song,
    //Changes the Non_Volatile Status of the Pokemon to Sleep after one round if possible.
    Yawn,
    //Is set after a flying type uses roost. This changes the flying type either to undefined, if
    //the Pokemon has two types, or to Normal if it has only one. Because of the possible
    //combination of normal and flying it is needed to have two indicators to determine which type
    //must be changed back.
    Roost_Type_One,
    Roost_Type_Two,
    //Attacks that deal damage at the end of every turn and binds the Pokemon -> It can not be
    //changed out. Lasts at least 2 and at most 5 turns.
    Trap,
}

///Print method for non volatile status changes.
pub fn print_non_volatile(status: Non_Volatile) -> String {
    match status {
        Non_Volatile::Undefined => String::from(""),
        Non_Volatile::Paralysis => String::from("paralysed"),
        Non_Volatile::Sleep => String::from("asleep"),
        Non_Volatile::Freeze => String::from("freezed"),
        Non_Volatile::Burn => String::from("burned"),
        _ => String::from("poisoned")
    }
}

///Enum for Genders
#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}


///Makes it easier to acces the Stats directly
enum_from_primitive! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Stats {
        Undefined = 0,
        Hp = 1,
        Attack = 2,
        Defense = 3,
        Special_Attack = 4,
        Special_Defense = 5,
        Speed = 6,
        Accuracy = 7,
        Evasion = 8,
    }
}

///Weather enum for the arena.
#[derive(Debug, Clone)]
pub enum Weather {
    Clear_Sky,
    Sunlight,
    //no need to handle it right now, only caused by abilities
    Harsh_Sunlight,
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

///All Flags that can be important for a move. Contains for example if a move is influenced by
///another move or condition the pokemon or arena is in.
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

///More or less randomly provides a gender for a pokemon given the distribution for the species.
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
