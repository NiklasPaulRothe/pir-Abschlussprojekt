// use db::pokemon_token::PokemonToken;


// /// The standard arena is based on the default XvX fight.

// impl super::Arena {
//     #[allow(dead_code)]
//     pub fn fight(&self) {



//         // Move der Spieler von Team 1 abfragen. (Methode in Player?)
//         // Move der Spieler von Team 2 abfragen. (Methode in Player?)
//             // Mögliche Moves:
//                 // Pokemon Tausch (Swap)
//                 // Attacke wählen
//             // Auswahlscreens müssen für jeden human zur verfügung gestellt werden.
//             // 1. Screen: Auswahl zwischen Swap und Attacke
//             // Swap Screen: Anzeigen der Pokemon mit HP und Status (Schlaf etc.) evtl. more infos
//             //              Button. Back Button
//             // Attack Screen: Auswahl der Attacken mit AP und Typ(Farbig hinterlegt?) Back Button

//         // Wenn ein Spieler sein Pokemon tauschen möchte wird dies zu erst gemacht. Bei mehreren
//         // in der Reihenfolge T1[0,1,2...] dann T2[0,1,2...]


//         // Entscheidung zwischen Attackentypen:
//             // Prioattacken zu erst (Ruckzuckhieb z.B.)
//             // Dann Normale Attacken mit init wert


//         // Diese methode wird die Moves der einzelnen Spieler abfragen und diese abspeichern.
//         // Anschließend wird von hier aus in der jeweiligen Reihenfolge swap und battle aufgerufen.


//         // Von hier müsste auch die UI gesteuert werden. Zumindest in meinen Augen
//     }
// }
// #[allow(dead_code)]
// fn swap() {
//     // von hier aus wird der tausch des aktiven pokemon abgehandelt.
// }

// #[allow(dead_code)]
// fn battle(pokemon_one: &PokemonToken, pokemon_two: &PokemonToken) {
//     // Hier wird zunächst auf die Priorität der Attacken und dann auf den Init Wert der pokemon
//     // geprüft. Dann wird jeweils math aufgerufen.
//     // Noch unklar: Schlaf, Paralyse, Vernarrtheit, etc.
//     // XvX Kampf mit Area Attack wie Surfer: Attacke als Attacke gegen ein Pokemon ansehen und math
//     // öfters aufrufen?
// }
