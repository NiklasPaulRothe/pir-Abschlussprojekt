extern crate conrod;
extern crate find_folder;

use arena;
use conrod::backend::piston::event::UpdateEvent;
use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
use db;
use db::enums::Player;
use player;

// Constant window/button sizes
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const BUTTON_W: f64 = 150.0;
const BUTTON_H: f64 = 30.0;

/// Collection of all possible screens
#[derive(Clone, Debug)]
pub enum Screen {
    Title,
    Play,
    End,
    ChooseTeam,
    Battle,
    BattleText,
    BattleAttack,
    Switch,
    None,
}

#[derive(Clone)]
/// Available gamemodes
enum Mode {
    Singleplayer,
    Multiplayer,
}

/// App struct, which contains important data
///     ===== UI =====
///     screen          -   the current screen to be displayed
///     sub_screen      -   current partial screen to be displayed
///     label_color     -   text color
///     bg_color        -   background
///     button_color    -   button color
///     border_color    -   border colog
///
///     ===== Variables =====
///     pokedex         -   used Pokedex
///     pkmn_team       -   variable to save current team in team selection
///     sel_pkmn        -   currently selected pokemon and position in team
///     movedex         -   used Movedex
///     techs           -   all techniques of the currently selected pokemon
///     pkmn_moves      -   selected moves for the selected pokemon
///     player          -   saves the currently active player (One or Two)
///     mode            -   saves the currently active mode (Single- or Multiplayer)
///     done            -   determines if a round is done
///     changed_pkmn_p1 -   index of player one's last changed pokemon
///     changed_pkmn_p2 -   index of player two's last changed pokemon
///     battle_text     -   text that is shown during battle
#[derive(Clone)]
pub struct App {
    // UI
    screen: Screen,
    sub_screen: Screen,
    label_color: conrod::Color,
    bg_color: conrod::Color,
    button_color: conrod::Color,
    border_color: conrod::Color,

    // Variables
    pokedex: db::pokedex::Pokedex,
    pkmn_team: Vec<db::pokemon_token::PokemonToken>,
    sel_pkmn: (Option<db::pokemon_token::PokemonToken>, Option<usize>),
    movedex: db::movedex::Movedex,
    techs: Option<Vec<db::moves::Technique>>,
    pkmn_moves: Vec<db::moves::Technique>,
    player: Player,
    mode: Mode,
    done: bool,
    changed_pkmn_p1: usize,
    changed_pkmn_p2: usize,
    battle_text: String,
}
impl App {
    // Creates a new App with default settings
    pub fn new() -> Self {
        App {
            // UI
            screen: Screen::Title,
            sub_screen: Screen::None,
            label_color: conrod::color::BLACK,
            bg_color: conrod::color::WHITE,
            button_color: conrod::color::LIGHT_GREY,
            border_color: conrod::color::DARK_GREY,

            // Variables
            pokedex: db::pokedex::Pokedex::new(),
            pkmn_team: Vec::new(),
            sel_pkmn: (None, None),
            movedex: db::movedex::Movedex::new(),
            techs: None,
            pkmn_moves: Vec::new(),
            player: Player::One,
            mode: Mode::Singleplayer,
            done: false,
            changed_pkmn_p1: 0,
            changed_pkmn_p2: 0,
            battle_text: String::new(),
        }
    }

    // Gets the current screen
    pub fn set_screen(&mut self, screen: Screen) {
        self.screen = screen
    }

    pub fn set_dead_switch(&mut self, screen: Screen, player: db::enums::Player) {
        self.screen = screen;
        self.player = player;
    }

    // Resets all the variables
    fn reset(&mut self) {
        self.pokedex = db::pokedex::Pokedex::new();
        self.pkmn_team = Vec::new();
        self.sel_pkmn = (None, None);
        self.movedex = db::movedex::Movedex::new();
        self.techs = None;
        self.pkmn_moves = Vec::new();
        self.player = Player::One;
        self.mode = Mode::Singleplayer;
        self.done = false;
        self.changed_pkmn_p1 = 0;
        self.changed_pkmn_p2 = 0;
        self.battle_text = String::new();
    }

    // Gets the current screen
    pub fn get_screen(&self) -> Screen {
        self.clone().screen
    }

    fn switch_player(&mut self) {
        match self.player {
            Player::One => self.player = Player::Two,
            Player::Two => self.player = Player::One,
        }
    }

    // Returns the changed pokemon of a given player
    pub fn get_changed_pokemon(&mut self, player: db::enums::Player) -> usize {
        self.screen = Screen::Switch;

        match player {
            Player::One => {
                self.player = Player::One;
                return self.changed_pkmn_p1;
            }
            Player::Two => {
                self.player = Player::Two;
                return self.changed_pkmn_p2;
            }
        }
    }

    // Sets the battle text
    pub fn set_battle_text(&mut self, text: String) {
        self.battle_text = [self.battle_text.clone(), " ".to_string(), text, "\n".to_string()]
            .concat();
    }

    // Draws the UI
    pub fn draw_window<'a>(&mut self) {
        // Construct the window.
        let mut window: Window = piston::window::WindowSettings::new("PokemonBattleArena",
                                                                     [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .vsync(true)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build window: {}", e));

        // Create the event loop.
        let mut events = WindowEvents::new();

        // Construct the `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // Create Ids for every used widget
        let ids = Ids::new(ui.widget_id_generator());

        // Add a font from file
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .unwrap_or_else(|e| panic!("Failed to find folder: {}", e));
        let font_path = assets.join("fonts/arial/arial.ttf");
        ui.fonts
            .insert_from_file(font_path)
            .unwrap_or_else(|e| panic!("Failed to get font: {}", e));

        // No text to draw -> create an empty text texture cache.
        let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

        // The image map describing each of our widget->image mappings (in this case none)
        let image_map = conrod::image::Map::new();

        let app = self;
        // Create arena with dummy players
        let mut player1 = player::Player::new();
        let mut player2 = player::Player::new();
        let mut arena = arena::Arena::new(&mut player1,
                                          &mut player2,
                                          db::enums::Types::Normal,
                                          db::enums::Weather::ClearSky);

        // Poll events from the window.
        while let Some(event) = window.next_event(&mut events) {
            // Convert the piston event to a conrod event.
            if let Some(e) = piston::window::convert_event(event.clone(), &window) {
                ui.handle_event(e);
            }

            event.update(|_| {
                let mut ui = &mut ui.set_widgets();

                // Create new empty canvas as default background
                widget::Canvas::new()
                    .border(0.0)
                    .color(app.bg_color)
                    .set(ids.canvas, ui);





                // /////////////////////////////////////////////////////////////
                // //////////                TITLE SCREEN             //////////
                // /////////////////////////////////////////////////////////////
                if let Screen::Title = app.screen {
                    // Play button
                    // Lets the user switch to the gamemode selection
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Play")
                        .label_color(app.label_color)
                        .middle_of(ids.canvas)
                        .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                        .set(ids.button_play, ui)
                        .was_clicked() {
                        println!("Play");
                        app.screen = Screen::Play;
                    }

                    // // Options button
                    // // Lets the user switch to the options menu (not implemented)
                    // if widget::Button::new()
                    //     .border(1.0)
                    //     .border_color(app.border_color)
                    //     .color(app.button_color)
                    //     .label("Options")
                    //     .label_color(app.label_color)
                    //     .down_from(ids.button_play, 0.0)
                    //     .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                    //     .set(ids.button_options, ui)
                    //     .was_clicked() {
                    //     app.screen = Screen::Options;
                    //     println!("Options");
                    // }

                    // Exit button
                    // Closes the game
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Exit")
                        .label_color(app.label_color)
                        .down_from(ids.button_play, 0.0)
                        .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                        .set(ids.button_exit, ui)
                        .was_clicked() {
                        ::std::process::exit(0);
                    }
                }





                // /////////////////////////////////////////////////////////////
                // //////////                PLAY SCREEN              //////////
                // /////////////////////////////////////////////////////////////
                if let Screen::Play = app.screen {
                    app.reset();
                    // Singleplayer button
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Singleplayer")
                        .label_color(app.label_color)
                        .middle_of(ids.canvas)
                        .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                        .set(ids.button_sp, ui)
                        .was_clicked() {
                        println!("Singleplayer");
                        app.mode = Mode::Singleplayer;
                        app.screen = Screen::ChooseTeam;
                    }

                    // Multiplayer button
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Multiplayer")
                        .label_color(app.label_color)
                        .down_from(ids.button_sp, 0.0)
                        .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                        .set(ids.button_mp, ui)
                        .was_clicked() {
                        println!("Multiplayer");
                        app.mode = Mode::Multiplayer;
                        app.screen = Screen::ChooseTeam;
                    }

                    // Back button
                    // Returns to previous screen
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Back")
                        .label_color(app.label_color)
                        .down_from(ids.button_mp, 0.0)
                        .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                        .set(ids.button_back, ui)
                        .was_clicked() {
                        println!("Back");
                        app.screen = Screen::Title;
                    }
                }





                // // /////////////////////////////////////////////////////////////
                // // //////////              OPTIONS SCREEN             //////////
                // // /////////////////////////////////////////////////////////////
                // ===== not implemented =====
                // if let Screen::Options = app.screen {
                //     if widget::Button::new()
                //         .border(1.0)
                //         .border_color(app.border_color)
                //         .color(app.button_color)
                //         .label("Back")
                //         .label_color(app.label_color)
                //         .down_from(ids.button_mp, 0.0)
                //         .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                //         .set(ids.button_back, ui)
                //         .was_clicked() {
                //         println!("Back");
                //         app.screen = Screen::Title;
                //     }
                // }





                // /////////////////////////////////////////////////////////////
                // //////////            CHOOSE TEAM SCREEN           //////////
                // /////////////////////////////////////////////////////////////
                if let Screen::ChooseTeam = app.screen {
                    // =========================================================
                    // ==========           POKEDEX                   ==========
                    // =========================================================
                    let pokedex_entries = app.pokedex.get_entries();
                    let num_items = pokedex_entries.len();
                    let (mut events, scrollbar) = widget::ListSelect::single(num_items, 64.0)
                        .scrollbar_next_to()
                        .w_h(200.0, 650.0)
                        .mid_left_with_margin_on(ids.canvas, 25.0)
                        .scrollbar_width(15.0)
                        .set(ids.slist_pkmn, ui);

                    // Instantiate the scrollbar for the list.
                    if let Some(s) = scrollbar {
                        s.set(ui);
                    }

                    // Handle the `ListSelect`s events.
                    while let Some(event) = events.next(ui, |i| {
                        ::std::collections::HashSet::<usize>::new().contains(&i)
                    }) {
                        use conrod::widget::list_select::Event;

                        match event {
                            // For the `Item` events we instantiate the `List`'s items.
                            Event::Item(item) => {
                                let label = &pokedex_entries[item.i].get_name();

                                // Button with Pokemons' name as label
                                let button = widget::Button::new()
                                    .border(1.0)
                                    .border_color(app.bg_color)
                                    .color(app.button_color)
                                    .label(label)
                                    .label_color(app.label_color);
                                item.set(button, ui);
                            }

                            // When the button is pressed the repective Pokemon is selected
                            Event::Selection(selection) => {
                                // Save selected Pokemon and set its team index to none
                                app.sel_pkmn =
                                    (Some(db::pokemon_token::PokemonToken::from_model(app.pokedex
                                         .pokemon_by_id(selection + 1)
                                         .unwrap())),
                                     None);
                                // Create move list for the selected Pokemon
                                app.techs = Some(app.sel_pkmn
                                    .clone()
                                    .0
                                    .unwrap()
                                    .get_moves(app.movedex.clone())
                                    .get_entries());
                                // Reset the currently selected moves
                                app.pkmn_moves = Vec::new();
                            }
                            _ => {}
                        }
                    }



                    // =========================================================
                    // ==========              PLAYER TEAM            ==========
                    // =========================================================
                    let (mut events, _) = widget::ListSelect::single(6, 650.0 / 6.0)
                        .w_h(200.0, 650.0)
                        .mid_right_with_margin_on(ids.canvas, 25.0)
                        .set(ids.slist_team, ui);

                    // Handle the `ListSelect`s events.
                    while let Some(event) = events.next(ui, |i| {
                        ::std::collections::HashSet::<usize>::new().contains(&i)
                    }) {
                        use conrod::widget::list_select::Event;

                        match event {
                            // For the `Item` events we instantiate the `List`'s items.
                            Event::Item(item) => {
                                // `label` is either the name of the pokemon (if available)
                                // or - (if not)
                                let label = if &app.pkmn_team.len() > &item.i {
                                    app.pkmn_team.clone()[item.i].get_name()
                                } else {
                                    "".to_string()
                                };

                                // each list item is a button with `label` as label
                                let button = widget::Button::new()
                                    .border(1.0)
                                    .border_color(app.bg_color)
                                    .color(app.button_color)
                                    .label(&label)
                                    .label_color(app.label_color);
                                item.set(button, ui);
                            }

                            Event::Selection(selection) => {
                                println!("selected index (Team): {:?}", selection);
                                if selection < app.pkmn_team.len() {
                                    app.sel_pkmn = (Some(app.pkmn_team[selection].clone()),
                                                    Some(selection));
                                    app.techs = Some(app.sel_pkmn
                                        .clone()
                                        .0
                                        .unwrap()
                                        .get_moves(app.movedex.clone())
                                        .get_entries());
                                    app.pkmn_moves = Vec::new();

                                    if let Some(att) = app.sel_pkmn
                                        .clone()
                                        .0
                                        .unwrap()
                                        .get_move_one() {
                                        app.pkmn_moves.push(att);
                                    }
                                    if let Some(att) = app.sel_pkmn
                                        .clone()
                                        .0
                                        .unwrap()
                                        .get_move_two() {
                                        app.pkmn_moves.push(att);
                                    }
                                    if let Some(att) = app.sel_pkmn
                                        .clone()
                                        .0
                                        .unwrap()
                                        .get_move_three() {
                                        app.pkmn_moves.push(att);
                                    }
                                    if let Some(att) = app.sel_pkmn
                                        .clone()
                                        .0
                                        .unwrap()
                                        .get_move_four() {
                                        app.pkmn_moves.push(att);
                                    }
                                } else {
                                    println!("Error: No Pokemon here");
                                }
                            }
                            // Do nothing for every other event
                            _ => {}
                        }
                    }



                    // =========================================================
                    // ==========             DESCRIPTION             ==========
                    // =========================================================
                    let description = match app.sel_pkmn.0 {
                        None => "".to_string(),
                        Some(ref pkmn) => {
                            // === Description ===
                            let types = match pkmn.get_types() {
                                (type1, db::enums::Types::Undefined) => type1.to_string(),
                                (type1, type2) => {
                                    [type1.to_string(), "/".to_string(), type2.to_string()]
                                        .concat()
                                        .to_string()
                                }
                            };

                            ["#",
                             &pkmn.get_id().to_string(),
                             " ",
                             &pkmn.get_name(),
                             "\n\nType: ",
                             &types,
                             "\n\n",
                             &pkmn.get_description()]
                                .concat()
                        }
                    };

                    // Background for description
                    widget::Canvas::new()
                        .color(conrod::color::LIGHT_GREY)
                        .border(0.0)
                        .w_h((WIDTH as f64 / 3.0), (HEIGHT as f64 / 3.0))
                        .top_right_with_margins_on(ids.canvas, 35.0, WIDTH as f64 / 5.0)
                        .set(ids.bg_description, ui);

                    // Text-Widget to display description
                    widget::Text::new(&description)
                        .color(app.label_color)
                        .middle_of(ids.bg_description)
                        .align_text_left()
                        .font_size(15)
                        .padded_wh_of(ids.bg_description, 20.0)
                        .line_spacing(2.5)
                        .set(ids.text_sel_pkmn, ui);



                    // =========================================================
                    // ==========                SPRITE               ==========
                    // =========================================================
                    // Background for Sprite
                    widget::Canvas::new()
                        .color(conrod::color::LIGHT_GREY)
                        .border(0.0)
                        .w_h((WIDTH as f64 / 4.0), (HEIGHT as f64 / 3.0))
                        .top_left_with_margins_on(ids.canvas, 35.0, WIDTH as f64 / 5.0)
                        .set(ids.bg_sprite, ui);



                    // =========================================================
                    // ==========               ATTACKS               ==========
                    // =========================================================

                    // Background for attack selection
                    widget::Canvas::new()
                        .color(conrod::color::LIGHT_GREY)
                        .border(0.0)
                        .w_h(770.0, 320.0)
                        .mid_bottom_with_margin_on(ids.canvas, 100.0)
                        .set(ids.bg_att_sel, ui);

                    // Only show when there is a Pokemon selected
                    if let Some(_) = app.sel_pkmn.clone().0 {
                        let techniques = app.techs.clone().unwrap();
                        let num_items = techniques.len();

                        // List with all possible attacks for the selected Pokemon
                        let (mut events, scrollbar) = widget::ListSelect::single(num_items, 64.0)
                            .scrollbar_next_to()
                            .w_h(200.0, 320.0)
                            .mid_left_with_margin_on(ids.bg_att_sel, 0.0)
                            .scrollbar_width(15.0)
                            .set(ids.slist_att, ui);

                        // Instantiate the scrollbar for the list.
                        if let Some(s) = scrollbar {
                            s.set(ui);
                        }

                        // Handle the `ListSelect`s events.
                        while let Some(event) = events.next(ui, |i| {
                            ::std::collections::HashSet::<usize>::new().contains(&i)
                        }) {
                            use conrod::widget::list_select::Event;

                            match event {
                                // For the `Item` events we instantiate the `List`'s items.
                                Event::Item(item) => {
                                    let label = &techniques[item.i].get_name();

                                    // each button with the respective attack name as label
                                    let button = widget::Button::new()
                                        .border(1.0)
                                        .border_color(app.button_color)
                                        .color(app.bg_color)
                                        .label(label)
                                        .label_color(app.label_color);
                                    item.set(button, ui);
                                }

                                // Add attack to list when pressed
                                Event::Selection(selection) => {
                                    println!("selected index (Attack): {:?}", selection);

                                    // Moves have to be unique
                                    let mut allowed = true;
                                    for att in app.pkmn_moves.clone() {
                                        if att.get_id() == techniques[selection].get_id() {
                                            allowed = false
                                        }
                                    }

                                    if app.pkmn_moves.len() < 4 && allowed {
                                        app.pkmn_moves.push(techniques[selection].clone())
                                    } else {
                                        println!("Error: Pokemon's moves have to be different and can only be 4");
                                    }
                                }
                                _ => {}
                            }
                        }

                        // Get all attack names
                        let mut label = Vec::with_capacity(4);
                        for attack in app.pkmn_moves.clone() {
                            label.push(attack.get_name().to_string());
                        }
                        while label.len() < 4 {
                            label.push("".to_string());
                        }

                        // Set buttons with attacks
                        if widget::Button::new()
                            .border(4.0)
                            .border_color(app.button_color)
                            .color(app.bg_color)
                            .label(&label[0])
                            .label_color(app.label_color)
                            .left_from(ids.button_att2, 0.0)
                            .w_h(285.0, 160.0)
                            .set(ids.button_att1, ui)
                            .was_clicked() {
                            println!("Att Button 1");
                            if app.pkmn_moves.len() > 0 {
                                app.pkmn_moves.remove(0);
                            }
                        }
                        if widget::Button::new()
                            .border(4.0)
                            .border_color(app.button_color)
                            .color(app.bg_color)
                            .label(&label[1])
                            .label_color(app.label_color)
                            .top_right_with_margin_on(ids.bg_att_sel, 0.0)
                            .w_h(285.0, 160.0)
                            .set(ids.button_att2, ui)
                            .was_clicked() {
                            println!("Att Button 2");
                            if app.pkmn_moves.len() > 1 {
                                app.pkmn_moves.remove(1);
                            }
                        }
                        if widget::Button::new()
                            .border(4.0)
                            .border_color(app.button_color)
                            .color(app.bg_color)
                            .label(&label[2])
                            .label_color(app.label_color)
                            .down_from(ids.button_att1, 0.0)
                            .w_h(285.0, 160.0)
                            .set(ids.button_att3, ui)
                            .was_clicked() {
                            println!("Att Button 3");
                            if app.pkmn_moves.len() > 2 {
                                app.pkmn_moves.remove(2);
                            }
                        }
                        if widget::Button::new()
                            .border(4.0)
                            .border_color(app.button_color)
                            .color(app.bg_color)
                            .label(&label[3])
                            .label_color(app.label_color)
                            .right_from(ids.button_att3, 0.0)
                            .w_h(285.0, 160.0)
                            .set(ids.button_att4, ui)
                            .was_clicked() {
                            println!("Att Button 4");
                            if app.pkmn_moves.len() > 3 {
                                app.pkmn_moves.remove(3);
                            }
                        }
                    }

                    // Button to put current Pokemon into team
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Select")
                        .label_color(app.label_color)
                        .left_from(ids.button_fight, 75.0)
                        .w_h(BUTTON_W, BUTTON_H)
                        .set(ids.button_select, ui)
                        .was_clicked() {
                        println!("Select");

                        if !app.pkmn_moves.is_empty() {
                            match app.sel_pkmn.clone() {
                                (Some(mut pkmn), None) => {
                                    pkmn.set_moves(app.pkmn_moves.clone());
                                    app.pkmn_team.push(pkmn.clone());
                                    app.sel_pkmn = (None, None);
                                }
                                (Some(mut pkmn), Some(index)) => {
                                    pkmn.set_moves(app.pkmn_moves.clone());
                                    app.pkmn_team.remove(index);
                                    app.pkmn_team.insert(index, pkmn.clone());
                                    app.sel_pkmn = (None, None);
                                }
                                _ => println!("Error: No Pokemon selected"),
                            }
                        } else {
                            println!("Error: Pokemon has no moves");
                        }         
                    }

                    // Button to remove selected Pokemon from team
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Remove")
                        .label_color(app.label_color)
                        .right_from(ids.button_back, 75.0)
                        .w_h(BUTTON_W, BUTTON_H)
                        .set(ids.button_remove, ui)
                        .was_clicked() {
                        println!("Remove");
                        if let Some(i) = app.sel_pkmn.1 {
                            app.pkmn_team.remove(i);
                            app.sel_pkmn = (None, None);
                        } else {
                            println!("Error: No Pokemon at this position")
                        }
                    }

                    // Back-Button
                    // version erstellen die zu player one zurück geht
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Back")
                        .label_color(app.label_color)
                        .bottom_left_with_margins_on(ids.canvas, 35.0, 255.0)
                        .w_h(BUTTON_W, BUTTON_H)
                        .set(ids.button_back, ui)
                        .was_clicked() {
                        println!("Back");
                        app.screen = Screen::Play;
                    }

                    // Show different button depending on current mode and player
                    match (app.mode.clone(), app.player.clone()) {
                        // Singleplayer
                        (Mode::Singleplayer, _) => {
                            if widget::Button::new()
                                .border(1.0)
                                .border_color(app.border_color)
                                .color(app.button_color)
                                .label("Fight")
                                .label_color(app.label_color)
                                .bottom_right_with_margins_on(ids.canvas, 35.0, 255.0)
                                .w_h(BUTTON_W, BUTTON_H)
                                .set(ids.button_fight, ui)
                                .was_clicked() {
                                println!("Fight");

                                if !app.pkmn_team.is_empty() {
                                    arena.get_player_one().set_pokemon_list(app.pkmn_team.clone());
                                    arena.get_player_two().set_pokemon_list(app.pkmn_team.clone());
                                    arena.get_player_two().set_player_type(player::PlayerType::SimpleAi);

                                    app.player = Player::One;
                                    app.screen = Screen::Battle;
                                } else {
                                    println!("Error: Empty team");
                                }
                            }
                        }
                        // Multiplayer - Player One
                        (Mode::Multiplayer, Player::One) => {
                            if widget::Button::new()
                                .border(1.0)
                                .border_color(app.border_color)
                                .color(app.button_color)
                                .label("Player Two")
                                .label_color(app.label_color)
                                .bottom_right_with_margins_on(ids.canvas, 35.0, 255.0)
                                .w_h(BUTTON_W, BUTTON_H)
                                .set(ids.button_fight, ui)
                                .was_clicked() {
                                println!("Player Two");

                                if !app.pkmn_team.is_empty() {
                                    arena.get_player_one().set_pokemon_list(app.pkmn_team.clone());

                                    app.pkmn_team = Vec::new();
                                    app.sel_pkmn = (None, None);
                                    app.techs = None;
                                    app.player = Player::Two;
                                    app.screen = Screen::ChooseTeam;
                                } else {
                                    println!("Error: Empty team");
                                }     
                            }
                        }
                        // Multiplayer - Player Two
                        (Mode::Multiplayer, Player::Two) => {
                            if widget::Button::new()
                                .border(1.0)
                                .border_color(app.border_color)
                                .color(app.button_color)
                                .label("Fight")
                                .label_color(app.label_color)
                                .bottom_right_with_margins_on(ids.canvas, 35.0, 255.0)
                                .w_h(BUTTON_W, BUTTON_H)
                                .set(ids.button_fight, ui)
                                .was_clicked() {
                                println!("Fight");

                                if !app.pkmn_team.is_empty() {
                                    arena.get_player_two().set_pokemon_list(app.pkmn_team.clone());

                                    app.player = Player::One;
                                    app.screen = Screen::Battle;
                                } else {
                                    println!("Error: Empty team");
                                }     
                            }
                        }
                    }
                }





                // /////////////////////////////////////////////////////////////
                // //////////               BATTLE SCREEN             //////////
                // /////////////////////////////////////////////////////////////
                if let Screen::Battle = app.screen {
                    // ===== Variables =====
                    // Current Player
                    let player_show = match app.player {
                        Player::One => "Player One",
                        Player::Two => "Player Two",
                    };

                    // Player1:
                    let player1 = arena.get_player_one().clone();
                    // Highlight Pokemon text when it is its turn
                    let color1 = match app.player {
                        Player::One => conrod::color::YELLOW,
                        _ => conrod::color::WHITE,
                    };
                    // Current Pokemon of player1
                    let current1 = player1.clone()
                        .get_pokemon_list()[player1.clone().get_current()]
                        .clone();
                    // ...its name
                    let name1 = current1.clone()
                        .get_name();
                    // ...its hp
                    let hp1 = current1.clone()
                        .get_current()
                        .get_stat(&db::enums::Stats::Hp)
                        .to_string();
                    // ...its max hp
                    let max_hp1 = current1.clone()
                        .get_base()
                        .get_stat(&db::enums::Stats::Hp)
                        .to_string();
                    // ...its status
                    let status1 = current1.clone()
                        .get_non_volatile()
                        .0
                        .to_string();
                    let text1 = [
                        name1, 
                        "\n\nHP: ".to_string(), 
                        hp1,
                        " / ".to_string(), 
                        max_hp1, 
                        " \n".to_string(), 
                        status1.to_string()
                        ].concat();


                    // Player2:
                    let player2 = arena.get_player_two().clone();
                    // Highlight Pokemon text when it is its turn
                     let color2 = match app.player {
                        Player::Two => conrod::color::YELLOW,
                        _ => conrod::color::WHITE,
                    };
                    // Current Pokemon of player2
                    let current2 = player2.clone()
                        .get_pokemon_list()[player1.clone().get_current()]
                        .clone();
                    // ...its name
                    let name2 = current2.clone()
                        .get_name();
                    // ...its hp
                    let hp2 = current2.clone()
                        .get_current()
                        .get_stat(&db::enums::Stats::Hp)
                        .to_string();
                    // ...its max hp
                    let max_hp2 = current2.clone()
                        .get_base()
                        .get_stat(&db::enums::Stats::Hp)
                        .to_string();
                    // ...its status
                    let status2 = current2.clone()
                        .get_non_volatile()
                        .0
                        .to_string();
                    let text2 = [
                        name2,
                        "\n\nHP: ".to_string(), 
                        hp2, 
                        " / ".to_string(),
                        max_hp2, 
                        " \n".to_string(), 
                        status2.to_string()
                        ].concat();


                    // ===== Canvases =====
                    // Battle Text BG
                    widget::Canvas::new()
                        .color(app.button_color)
                        .border_color(app.border_color)
                        .border(2.0)
                        .w_h(WIDTH as f64 - 350.0, 240.0)
                        .bottom_left_of(ids.canvas)
                        .set(ids.bg_text, ui);
                    
                    // BG Pokemon1
                    widget::Canvas::new()
                        .color(conrod::color::LIGHT_BLUE)
                        .border(0.0)
                        .w_h(300.0, 350.0)
                        .bottom_left_with_margins_on(ids.canvas, 250.0, 10.0)
                        .set(ids.bg_sprite, ui);

                    // BG Pokemon2
                    widget::Canvas::new()
                        .color(conrod::color::LIGHT_PURPLE)
                        .border(0.0)
                        .w_h(300.0, 350.0)
                        .bottom_right_with_margins_on(ids.canvas, 250.0, 10.0)
                        .set(ids.bg_sprite2, ui);

                    // BG What to do next
                    widget::Canvas::new()
                        .color(conrod::color::LIGHT_GREEN)
                        .border(2.0)
                        .w_h(350.0, 240.0)
                        .bottom_right_of(ids.canvas)
                        .set(ids.bg_whatdo, ui);


                    // ===== Texts =====
                    // Show current player
                    widget::Text::new(&["Current Player: ", player_show].concat())
                        .color(app.label_color)
                        .mid_top_with_margin_on(ids.canvas, 35.0)
                        .align_text_left()
                        .font_size(25)
                        .line_spacing(10.0)
                        .set(ids.text_player, ui);
                    
                    // Show Battle Text
                    widget::Text::new(&app.battle_text)
                        .color(app.label_color)
                        .middle_of(ids.bg_text)
                        .align_text_left()
                        .font_size(20)
                        .padded_wh_of(ids.bg_text, 20.0)
                        .line_spacing(10.0)
                        .set(ids.text_battle, ui);

                    // Show Pokemon1
                    widget::Text::new(&text1)
                        .color(color1)
                        .middle_of(ids.bg_sprite)
                        .align_text_left()
                        .font_size(25)
                        .padded_wh_of(ids.bg_sprite, 20.0)
                        .line_spacing(10.0)
                        .set(ids.text_pkmn1, ui);

                    // Show Pokemon2
                    widget::Text::new(&text2)
                        .color(color2)
                        .middle_of(ids.bg_sprite2)
                        .align_text_left()
                        .font_size(25)
                        .padded_wh_of(ids.bg_sprite2, 20.0)
                        .line_spacing(10.0)
                        .set(ids.text_pkmn2, ui);  


                    // ===== Buttons =====
                    // Button to show moves
                    if widget::Button::new()
                        .border(2.0)
                        .color(app.button_color)
                        .border_color(app.border_color)
                        .label("Fight")
                        .label_color(app.label_color)
                        .mid_top_of(ids.bg_whatdo)
                        .w_h(350.0, 120.0)
                        .set(ids.button_att, ui)
                        .was_clicked() {
                        println!("Battle_Fight");

                        // If next move is already full (a move is charging) pokemon cannot du another
                        // one
                        match app.player {
                            Player::One => {
                                if let Some(_) = player1.clone().get_next_move() {
                                    app.player = Player::Two;
                                    println!("p1 nextm");
                                }
                            }
                            Player::Two => {
                                if let Some(_) = player2.clone().get_next_move() {
                                    app.player = Player::One;
                                    println!("p2 nextm");
                                }
                            }
                        }
                        if let Screen::BattleAttack = app.sub_screen {
                            app.sub_screen = Screen::None;
                        } else {
                            app.sub_screen = Screen::BattleAttack;
                        }
                    }

                    // Button to change Pokemon
                    if widget::Button::new()
                        .border(2.0)
                        .color(app.button_color)
                        .border_color(app.border_color)
                        .label("Pokémon")
                        .label_color(app.label_color)
                        .down_from(ids.button_att, 0.0)
                        .w_h(350.0, 120.0)
                        .set(ids.button_switch, ui)
                        .was_clicked() {
                        println!("Pokémon");
                        app.screen = Screen::Switch;
                    }



                    // /////////////////////////////////////////////////////////////
                    // //////////               CHOOSE ATTACK             //////////
                    // /////////////////////////////////////////////////////////////
                    if let Screen::BattleAttack = app.sub_screen {

                        // Set labels of attack buttons
                        let mut label = Vec::with_capacity(4);
                        let player = match app.player {
                            Player::One => arena.get_player_one().clone(),
                            Player::Two => arena.get_player_two().clone(),
                        };
                        let index = player.clone().get_current();
                        let current = player.clone().get_pokemon_list()[index].clone();

                        if let Some(att) = current.clone().get_move_one() {
                            label.push(att.get_name().to_string());
                        }
                        if let Some(att) = current.clone().get_move_two() {
                            label.push(att.get_name().to_string());
                        }
                        if let Some(att) = current.clone().get_move_three() {
                            label.push(att.get_name().to_string());
                        }
                        if let Some(att) = current.clone().get_move_four() {
                            label.push(att.get_name().to_string());
                        }

                        while label.len() < 4 {
                            label.push("".to_string());
                        }



                        // =========================================================
                        // ==========           ATTACK SELECTION          ==========
                        // =========================================================
                        // Create buttons with respective attacks
                        if widget::Button::new()
                            .border(2.0)
                            .color(app.button_color)
                            .border_color(app.border_color)
                            .label(&label[0])
                            .label_color(app.label_color)
                            .top_left_of(ids.bg_text)
                            .w_h(465.0, 120.0)
                            .set(ids.button_att1, ui)
                            .was_clicked() {
                            println!("Attack 1");

                            // Find out which players turn it is
                            let player = match app.player.clone() {
                                Player::One => arena.get_player_one(),
                                Player::Two => {
                                    app.done = true;
                                    app.battle_text = "".to_string();
                                    arena.get_player_two()        
                                }
                            };

                            // Get attack
                            let att = player.clone()
                                .get_pokemon_list()[player.clone().get_current()]
                                .clone()
                                .get_move_one();

                            // If available set attack as next move
                            match att {
                                Some(att) => {
                                    player.set_next_move(Some(player::Next::Move(att)));
                                    app.switch_player();
                                }
                                None => {
                                    app.battle_text = "Not a move".to_string();
                                    println!("Error: No move");
                                }
                            }
                            app.sub_screen = Screen::None;
                        }

                        if widget::Button::new()
                            .border(2.0)
                            .color(app.button_color)
                            .border_color(app.border_color)
                            .label(&label[1])
                            .label_color(app.label_color)
                            .right_from(ids.button_att1, 0.0)
                            .w_h(465.0, 120.0)
                            .set(ids.button_att2, ui)
                            .was_clicked() {
                            println!("Attack 2");

                            let player = match app.player.clone() {
                                Player::One => arena.get_player_one(),
                                Player::Two => {
                                    app.done = true;
                                    app.battle_text = "".to_string();
                                    arena.get_player_two()        
                                }
                            };
                            let att = player.clone()
                                .get_pokemon_list()[player.clone().get_current()]
                                .clone()
                                .get_move_two();

                            match att {
                                Some(att) => {
                                    player.set_next_move(Some(player::Next::Move(att)));
                                    app.switch_player();
                                }
                                None => {
                                    app.battle_text = "Not a move".to_string();
                                    println!("Error: No move");
                                }
                            }
                            app.sub_screen = Screen::None;
                        }

                        if widget::Button::new()
                            .border(2.0)
                            .color(app.button_color)
                            .border_color(app.border_color)
                            .label(&label[2])
                            .label_color(app.label_color)
                            .down_from(ids.button_att1, 0.0)
                            .w_h(465.0, 120.0)
                            .set(ids.button_att3, ui)
                            .was_clicked() {
                            println!("Attack 3");

                            let player = match app.player.clone() {
                                Player::One => arena.get_player_one(),
                                Player::Two => {
                                    app.done = true;
                                    app.battle_text = "".to_string();
                                    arena.get_player_two()        
                                }
                            };
                            let att = player.clone()
                                .get_pokemon_list()[player.clone().get_current()]
                                .clone()
                                .get_move_three();

                            match att {
                                Some(att) => {
                                    player.set_next_move(Some(player::Next::Move(att)));
                                    app.switch_player();
                                }
                                None => {
                                    app.battle_text = "Not a move".to_string();
                                    println!("Error: No move");
                                }
                            }
                            app.sub_screen = Screen::None;
                        }

                        if widget::Button::new()
                            .border(2.0)
                            .color(app.button_color)
                            .border_color(app.border_color)
                            .label(&label[3])
                            .label_color(app.label_color)
                            .right_from(ids.button_att3, 0.0)
                            .w_h(465.0, 120.0)
                            .set(ids.button_att4, ui)
                            .was_clicked() {
                            println!("Attack 4");

                            let player = match app.player.clone() {
                                Player::One => arena.get_player_one(),
                                Player::Two => {
                                    app.done = true;
                                    app.battle_text = "".to_string();
                                    arena.get_player_two()        
                                }
                            };
                            let att = player.clone()
                                .get_pokemon_list()[player.clone().get_current()]
                                .clone()
                                .get_move_four();

                            match att {
                                Some(att) => {
                                    player.set_next_move(Some(player::Next::Move(att)));
                                    app.switch_player();
                                }
                                None => {
                                    app.battle_text = "Not a move".to_string();
                                    println!("Error: No move");
                                }
                            }
                            app.sub_screen = Screen::None;
                        }
                    }

                    // Check if a round is done (done when player two selected his next move)
                    // When done, start to fight
                    if app.done {
                        println!("fight!");
                        arena.fight(app);
                        app.done = false;
                    }

                    // Show end screen when fight is over (one teams' pokemon are all dead)
                    if arena.get_player_one().clone().get_alive_count() == 0 {
                        app.screen = Screen::End;
                        app.player = Player::Two;
                    } else if arena.get_player_two().clone().get_alive_count() == 0 {
                        app.screen = Screen::End;
                        app.player = Player::One;
                    }
                }




                // /////////////////////////////////////////////////////////////
                // //////////           CHANGE POKEMON SCREEN         //////////
                // /////////////////////////////////////////////////////////////
                if let Screen::Switch = app.screen {
                    let player_show = match app.player {
                        Player::One => "Player One",
                        Player::Two => "Player Two",
                    };
                    widget::Text::new(player_show)
                        .color(app.label_color)
                        .middle_of(ids.canvas)
                        .align_text_left()
                        .font_size(25)
                        .line_spacing(10.0)
                        .set(ids.text_player, ui);

                    let (mut events, _) = widget::ListSelect::single(6, 650.0 / 6.0)
                        .w_h(200.0, 650.0)
                        .mid_left_with_margin_on(ids.canvas, 25.0)
                        .set(ids.slist_team, ui);

                    let player = match app.player {
                        Player::One => arena.get_player_one(),
                        Player::Two => arena.get_player_two(),
                    };
                    let mut player_list = player.clone();
                    let pkmn_list = player_list.get_pokemon_list();

                    // Handle the `ListSelect`s events.
                    while let Some(event) = events.next(ui, |i| {
                        ::std::collections::HashSet::<usize>::new().contains(&i)
                    }) {
                        use conrod::widget::list_select::Event;

                        match event {
                            // For the `Item` events we instantiate the `List`'s items.
                            Event::Item(item) => {
                                // `label` is either the name of the pokemon (if available)
                                // or - (if not)
                                let label = if pkmn_list.len() > item.i {
                                    pkmn_list.clone()[item.i].get_name()
                                } else {
                                    "".to_string()
                                };

                                // each list item is a button with `label` as label
                                let button = widget::Button::new()
                                    .border(1.0)
                                    .border_color(app.bg_color)
                                    .color(app.button_color)
                                    .label(&label)
                                    .label_color(app.label_color);
                                item.set(button, ui);
                            }

                            Event::Selection(selection) => {
                                println!("selected index (Team): {:?}", selection);
                                if selection < pkmn_list.len() {
                                    match app.player {
                                        Player::One => {
                                            if selection != player.get_current() {
                                                app.changed_pkmn_p1 = selection;
                                                app.screen = Screen::Battle;
                                                app.player = Player::Two;
                                            } else {
                                                println!("Error: Can't swap with itself");
                                            }
                                        }
                                        Player::Two => {
                                            if selection != player.get_current() {
                                                app.changed_pkmn_p2 = selection;
                                                app.screen = Screen::Battle;
                                                app.player = Player::One;
                                            } else {
                                                println!("Error: Can't swap with itself");
                                            }
                                        }
                                    }
                                } else {
                                    println!("Error: No Pokemon here");
                                }
                            }
                            // Do nothing for every other event
                            _ => {}
                        }
                    }
                    // Back-Button
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Back")
                        .label_color(app.label_color)
                        .mid_bottom_with_margin_on(ids.canvas, 35.0)
                        .w_h(BUTTON_W, BUTTON_H)
                        .set(ids.button_back, ui)
                        .was_clicked() {
                        println!("Back");
                        app.screen = Screen::Battle;
                    }
                }




                // /////////////////////////////////////////////////////////////
                // //////////                 END SCREEN              //////////
                // /////////////////////////////////////////////////////////////
                if let Screen::End = app.screen {
                    let winner = match app.player {
                        Player::One => "P L A Y E R   O N E",
                        Player::Two => "P L A Y E R   T W O"
                    };           
                    widget::Text::new(&["      W I N N E R\n\n".to_string(), winner.to_string()].concat())
                        .color(app.label_color)
                        .mid_top_with_margin_on(ids.canvas, 100.0)
                        .align_text_left()
                        .font_size(25)
                        .line_spacing(10.0)
                        .set(ids.text_player, ui);

                    // Exit button
                    // Closes the game
                    if widget::Button::new()
                        .border(1.0)
                        .border_color(app.border_color)
                        .color(app.button_color)
                        .label("Exit")
                        .label_color(app.label_color)
                        .mid_bottom_with_margin_on(ids.canvas, 35.0)
                        .w_h(BUTTON_W * 2.0, BUTTON_H * 2.0)
                        .set(ids.button_exit, ui)
                        .was_clicked() {
                        ::std::process::exit(0);
                    }
                }

                // Confirm swap on death
                arena.get_player_one().set_current(app.changed_pkmn_p1);
                arena.get_player_two().set_current(app.changed_pkmn_p2);
            });

            window.draw_2d(&event,
                           |c, g| if let Some(primitives) = ui.draw_if_changed() {
                               fn texture_from_image<T>(img: &T) -> &T {
                                   img
                               };
                               piston::window::draw(c,
                                                    g,
                                                    primitives,
                                                    &mut text_texture_cache,
                                                    &image_map,
                                                    texture_from_image);
                           });
        }
    }
}



widget_ids! {
    struct Ids {
        // === canvas ===
        canvas,
        bg_description,
        bg_sprite,
        bg_sprite2,
        bg_att_sel,
        bg_text,
        bg_whatdo,


        // === selection_list ===
        slist_pkmn,
        slist_team,
        slist_att,

        // === text ===
        text_sel_pkmn,
        text_pkmn1,
        text_pkmn2,
        text_battle,
        text_hp1,
        text_hp2,
        text_player,

        tab_whatdo,
        tab_pokemon,
        tab_fight,

        // === buttons ===
        button_play,
        button_options,
        button_exit,
        button_sp,
        button_mp,
        button_back,
        button_fight,
        button_select,
        button_remove,
        button_att,
        button_att1,
        button_att2,
        button_att3,
        button_att4,
        button_switch,
    }
}
