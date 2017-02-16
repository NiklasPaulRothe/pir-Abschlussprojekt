extern crate num;
extern crate rand;

use self::rand::{Rng, thread_rng};

/// Enum for the pokemon/attack types.
/// Can be assigned from i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone, Eq, PartialEq, Hash, Copy)]
    pub enum Types {
        Normal = 1,
        Fighting = 2,
        Flying = 3,
        Poison = 4,
        Ground = 5,
        Rock = 6,
        Bug = 7,
        Ghost = 8,
        Steel = 9,
        Fire = 10,
        Water = 11,
        Grass = 12,
        Electric = 13,
        Psychic = 14,
        Ice = 15,
        Dragon = 16,
        Dark = 17,
        Fairy = 18,
        Undefined = 19,
    }
}

impl ::std::fmt::Display for Types {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let x = match *self {
            Types::Normal => "Normal",
            Types::Fighting => "Fighting",
            Types::Flying => "Flying",
            Types::Poison => "Poison",
            Types::Ground => "Ground",
            Types::Rock => "Rock",
            Types::Bug => "Bug",
            Types::Ghost => "Ghost",
            Types::Steel => "Steel",
            Types::Fire => "Fire",
            Types::Water => "Water",
            Types::Grass => "Grass",
            Types::Electric => "Electric",
            Types::Psychic => "Psychic",
            Types::Ice => "Ice",
            Types::Dragon => "Dragon",
            Types::Dark => "Dark",
            Types::Fairy => "Fairy",
            Types::Undefined => "Undefined",
        };

        write!(f, "{}", x)
    }
}

/// Enum for the Categories a move can have. They are used to get smaller samples of moves when
/// resolve their effects.
enum_from_primitive! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum MoveCategory {
        Damage = 0,
        Ailment = 1,
        NetGoodStats = 2,
        Heal = 3,
        DamageAndAilment = 4,
        Swagger = 5,
        DamageAndLower = 6,
        DamageAndRaise = 7,
        DamageAndHeal = 8,
        Ohko = 9,
        WholeFieldEffect = 10,
        FieldEffect = 11,
        ForceSwitch = 12,
        Unique = 13,
    }
}

/// All ailments that are known and can be caused by one or more moves.
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
    HealBlock,
    NoTypeImmunity,
    LeechSeed,
    Embargo,
    PerishSong,
    Ingrain,
}

/// All the major status Changes that can not be caused at the same time.
#[derive(Debug, Clone, PartialEq)]
pub enum NonVolatile {
    Undefined,
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison,
    BadPoison,
}
impl ::std::fmt::Display for NonVolatile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let x = match *self {
            NonVolatile::Paralysis => "Paralysis",
            NonVolatile::Sleep => "Sleep",
            NonVolatile::Freeze => "Freeze",
            NonVolatile::Burn => "Burn",
            NonVolatile::Poison => "Poison",
            NonVolatile::BadPoison => "Bad Poison",
            NonVolatile::Undefined => "",
        };

        write!(f, "{}", x)
    }
}

/// Print method for non volatile status changes.
pub fn print_non_volatile(status: NonVolatile) -> String {
    match status {
        NonVolatile::Undefined => String::from(""),
        NonVolatile::Paralysis => String::from("paralysed"),
        NonVolatile::Sleep => String::from("asleep"),
        NonVolatile::Freeze => String::from("freezed"),
        NonVolatile::Burn => String::from("burned"),
        _ => String::from("poisoned"),
    }
}

/// Flags that have a influence at the end of each turn.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndOfTurn {
    // absorbs some HP at the End of every Turn
    LeechSeed,
    // Counts from 0 to 4, one step each round, even in the turn it was initially used.
    // When 4 is reached the Pokemon faints. Counting only continues when the Pokemon is part of
    // the battle, but the counter will not be resetted if the Pokemon is changed.
    PerishSong,
    // Changes the NonVolatile Status of the Pokemon to Sleep after one round if possible.
    Yawn,
    // Is set after a flying type uses roost. This changes the flying type either to undefined, if
    // the Pokemon has two types, or to Normal if it has only one. Because of the possible
    // combination of normal and flying it is needed to have two indicators to determine which type
    // must be changed back.
    RoostTypeOne,
    RoostTypeTwo,
    // Attacks that deal damage at the end of every turn and binds the Pokemon -> It can not be
    // changed out. Lasts at least 2 and at most 5 turns.
    Trap,
    // Restores 1/16 of the Users HP at the end of every Turn.
    Ingrain,
}

/// Flags that need to be resolved before attacking
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Fighting {
    Confusion,
    Infatuation,
}

/// Flags that have a influence when resolving attacks.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Resolve {
    NoTypeImmunity,
    HealBlock,
    Telekinesis,
    Protect,
}

/// Flags that have a influence when choosing a move.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Choose {
    Torment,
}

/// Flags that influence one side of the field.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PlayerFlag {
    // lowers speed of Pokemon that are switched in.
    StickyWeb,
    // deals Damage to Pokemon that are switched in.
    StealthRock,
    // poisons Pokemon that are switched in.
    ToxicSpikes,
    // prevents opponents from landing criticla hits.
    LuckyChant,
    // deals Damage to Pokemon that are switched in.
    Spikes,
    CraftyShield,
    MatBlock,
    // protects the User from every move with Priority > 0 for one round.
    QuickGuard,
    WideGuard,
    // Doubles the speed for speed check.
    Tailwind,
    Safeguard,
    Reflect,
    LightScreen,
    Mist,
}

/// Enum for Genders
#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}

/// More or less randomly provides a gender for a pokemon given the distribution for the species.
pub fn get_gender(gender_rate: i8) -> Gender {
    let mut rng = thread_rng();
    let probability = rng.gen_range(0.0, 100.1);
    match gender_rate {
        -1 => Gender::Genderless,
        0 => Gender::Male,
        1 => {
            if probability < 87.5 {
                return Gender::Male;
            }
            Gender::Female
        }
        2 => {
            if probability < 75.0 {
                return Gender::Male;
            }
            Gender::Female
        }
        4 => {
            if probability < 50.0 {
                return Gender::Male;
            }
            Gender::Female
        }
        6 => {
            if probability < 25.0 {
                return Gender::Male;
            }
            Gender::Female
        }
        7 => {
            if probability < 12.5 {
                return Gender::Male;
            }
            Gender::Female
        }
        8 => Gender::Female,
        _ => Gender::Genderless,
    }
}

/// Makes it easier to acces the Stats directly
enum_from_primitive! {
    #[derive(Debug, Clone, PartialEq, Copy)]
    pub enum Stats {
        Undefined = 0,
        Hp = 1,
        Attack = 2,
        Defense = 3,
        SpecialAttack = 4,
        SpecialDefense = 5,
        Speed = 6,
        Accuracy = 7,
        Evasion = 8,
    }
}

/// Weather enum for the arena.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub enum Weather {
    ClearSky,
    Sunlight,
    // no need to handle it right now, only caused by abilities
    HarshSunlight,
    Rain,
    // no need to handle it right now, only caused by abilities
    HeavyRain,
    Sandstorm,
    Hail,
    // no need to handle it right now, only caused by abilities
    AirCurrent,
}

/// Enum for the Damage Class of a attack.
/// Can be assigned from a i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone, PartialEq)]
    pub enum DamageClass {
        Physical = 1,
        Special = 2,
        Status = 3,
    }
}

/// Enum that contains the valid target(s) of a move.
/// Can be assigned from a i32 value.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone, PartialEq)]
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

/// All Flags that can be important for a move. Contains for example if a move is influenced by
/// another move or condition the pokemon or arena is in.
enum_from_primitive! {
    #[derive(Debug, RustcDecodable, Clone, PartialEq)]
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
        NonSkyBattle = 20,
    }
}

pub fn stat_to_string(stat: Stats) -> &'static str {
    match stat {
        Stats::Hp => "hp",
        Stats::Attack => "attack",
        Stats::Defense => "defense",
        Stats::SpecialAttack => "special attack",
        Stats::SpecialDefense => "special defense",
        Stats::Speed => "speed",
        Stats::Accuracy => "accuracy",
        Stats::Evasion => "evasion",
        _ => "",
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub enum FieldEffects {
    MudSport,
    WaterSport,
    Gravity,
    TrickRoom,
    WonderRoom,
    MagicRoom,
    IonDeluge,
    GrassyTerrain,
    MistyTerrain,
    ElectricTerrain,
    FairyLock,
}

impl FieldEffects {
    /// Returns the maximum amount of turns the given FieldEffect lasts.
    pub fn get_max_rounds(&self) -> u8 {
        match *self {
            FieldEffects::MudSport => 4,
            FieldEffects::WaterSport => 4,
            FieldEffects::Gravity => 4,
            FieldEffects::TrickRoom => 4,
            FieldEffects::WonderRoom => 4,
            FieldEffects::MagicRoom => 4,
            FieldEffects::IonDeluge => 0,
            FieldEffects::GrassyTerrain => 4,
            FieldEffects::MistyTerrain => 4,
            FieldEffects::ElectricTerrain => 4,
            FieldEffects::FairyLock => 1,
        }
    }
}
/// Player enum for representing a player e.g. in resolve
#[derive(Debug, Copy, Clone)]
pub enum Player {
    One,
    Two,
}
