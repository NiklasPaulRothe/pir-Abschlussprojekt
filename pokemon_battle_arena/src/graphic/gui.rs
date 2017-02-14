extern crate find_folder;
extern crate conrod;

use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;
use db;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const BUTTON_W: f64 = 150.0;
const BUTTON_H: f64 = 30.0;

#[derive(Clone)]
enum Screen {
    Title,
    Play,
    Options,
    ChooseTeam,
    BattleStart,
    BattleAttackSel,
    BattleSwap,
}

/// App struct, which contains important data
///     screen:         screen that gets drawn
///     label_color:    color of labels (text)
///     bg_color:       background color
///     pokedex:        currently used pokedex
///     pkmn_team:      current team
///     sel_pkmn:       currently selected pokemon and its index in the team
#[derive(Clone)]
struct App {
    screen: Screen,
    label_color: conrod::Color,
    bg_color: conrod::Color,
    pokedex: db::pokedex::Pokedex,
    pkmn_team: Vec<db::pokemon_token::PokemonToken>,
    sel_pkmn: (Option<db::pokemon_token::PokemonToken>, Option<usize>),
    movedex: db::movedex::Movedex,
    techs: Option<Vec<db::moves::Technique>>,
    pkmn_moves: Vec<db::moves::Technique>,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::Title,
            label_color: conrod::color::BLACK,
            bg_color: conrod::color::WHITE,
            pokedex: db::pokedex::Pokedex::new(),
            pkmn_team: Vec::new(),
            sel_pkmn: (None, None),
            movedex: db::movedex::Movedex::new(),
            techs: None,
            pkmn_moves: Vec::new(),
        }
    }

    fn set_screen(mut self, screen: Screen) {
        self.screen = screen
    }

    fn get_screen(&self) -> Screen {
        self.clone().screen
    }

    fn get_team(&self) -> Vec<db::pokemon_token::PokemonToken> {
        self.clone().pkmn_team
    }
}

pub fn draw_window() {
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
    ui.fonts.insert_from_file(font_path).unwrap_or_else(|e| panic!("Failed to get font: {}", e));

    // No text to draw -> create an empty text texture cache.
    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in this case none)
    let image_map = conrod::image::Map::new();

    let mut app = App::new();

    // Poll events from the window.
    while let Some(event) = window.next_event(&mut events) {
        // Convert the piston event to a conrod event.
        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

            let mut ui = &mut ui.set_widgets();

            // Create new empty canvas
            widget::Canvas::new()
                .border(0.0)
                .color(app.bg_color)
                .set(ids.canvas, ui);

            // draws Title-Screen
            // Contains:    Play-Button
            //              Options-Button
            //              Exit-Button
            if let Screen::Title = app.screen {
                // Play button
                // Shows Play-Screen when clicked
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Play")
                    .label_color(app.label_color)
                    .middle_of(ids.canvas)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_play, ui)
                    .was_clicked() {
                    println!("Play");
                    app.screen = Screen::Play;
                }

                // Options button
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Options")
                    .label_color(app.label_color)
                    .down_from(ids.button_play, 0.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_options, ui)
                    .was_clicked() {
                    app.screen = Screen::Options;
                    println!("Options");
                }

                // Exit button
                // closes the window
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Exit")
                    .label_color(app.label_color)
                    .down_from(ids.button_options, 0.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_exit, ui)
                    .was_clicked() {
                    ::std::process::exit(0);
                }
            }

            // draws Play-Screen
            // Contains:    Singleplayer-Button
            //              Multiplayer-Button
            //              Back-Button
            if let Screen::Play = app.screen {
                // Singleplayer button
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Singleplayer")
                    .label_color(app.label_color)
                    .middle_of(ids.canvas)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_sp, ui)
                    .was_clicked() {
                    app.screen = Screen::ChooseTeam;
                    println!("Singleplayer");
                }

                // Multiplayer button
                // not implemented yet
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Multiplayer")
                    .label_color(app.label_color)
                    .down_from(ids.button_sp, 0.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_mp, ui)
                    .was_clicked() {
                    println!("Multiplayer");
                }

                // Back button
                // returns to previous screen
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Back")
                    .label_color(app.label_color)
                    .down_from(ids.button_mp, 0.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_back, ui)
                    .was_clicked() {
                    println!("Back");
                    app.screen = Screen::Title;
                }
            }

            // draws Options-Screen
            if let Screen::Options = app.screen {
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Back")
                    .label_color(app.label_color)
                    .down_from(ids.button_mp, 0.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_back, ui)
                    .was_clicked() {
                    println!("Back");
                    app.screen = Screen::Title;
                }
            }

            // Draws Choose Team Screen
            // IDEAS:   - pictures of pokemon
            //          - filter gen/type (get pokedex with only specific type/gen)
            //          - search ( fn that gives Vec with pokemon that have certain string in name)
            if let Screen::ChooseTeam = app.screen {

                // ===============================================================
                // = Scrollable List from which the Player can select their team =
                // ===============================================================
                let pokedex_entries = app.pokedex.get_entries();
                let num_items = pokedex_entries.len();
                let item_h = 64.0;

                let (mut events, scrollbar) = widget::ListSelect::single(num_items, item_h)
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

                            // each button with the respective pokemon name as label
                            let button = widget::Button::new()
                                .border(1.0)
                                .border_color(conrod::color::WHITE)
                                .color(conrod::color::LIGHT_GREY)
                                .label(label)
                                .label_color(app.label_color);
                            item.set(button, ui);
                        }

                        // Select Pokemon when button is pressed
                        Event::Selection(selection) => {
                            println!("selected index (PokeDex): {:?}", selection);

                            app.sel_pkmn =
                                (Some(db::pokemon_token::PokemonToken::from_model(app.pokedex
                                     .pokemon_by_id(selection + 1)
                                     .unwrap())),
                                 None);
                            app.techs = Some(app.sel_pkmn
                                .clone()
                                .0
                                .unwrap()
                                .get_moves(app.movedex.clone())
                                .get_entries());
                            app.pkmn_moves = Vec::new();
                        }
                        _ => {}
                    }
                }

                // =====================================
                // = List that shows the player's team =
                // =====================================
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
                                .border_color(conrod::color::WHITE)
                                .color(conrod::color::LIGHT_GREY)
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

                                if let Some(att) = app.sel_pkmn.clone().0.unwrap().get_move_one() {
                                    app.pkmn_moves.push(att);
                                }
                                if let Some(att) = app.sel_pkmn.clone().0.unwrap().get_move_two() {
                                    app.pkmn_moves.push(att);
                                }
                                if let Some(att) = app.sel_pkmn
                                    .clone()
                                    .0
                                    .unwrap()
                                    .get_move_three() {
                                    app.pkmn_moves.push(att);
                                }
                                if let Some(att) = app.sel_pkmn.clone().0.unwrap().get_move_four() {
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

                // ===================================
                // = Description of selected Pokemon =
                // ===================================
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

                // ==================================
                // = Sprite of the selected Pokemon =
                // ==================================

                // Background for Sprite
                widget::Canvas::new()
                    .color(conrod::color::LIGHT_GREY)
                    .border(0.0)
                    .w_h((WIDTH as f64 / 4.0), (HEIGHT as f64 / 3.0))
                    .top_left_with_margins_on(ids.canvas, 35.0, WIDTH as f64 / 5.0)
                    .set(ids.bg_sprite, ui);

                // ====================
                // = Attack Selection =
                // ====================

                // Background for attack selection
                widget::Canvas::new()
                    .color(conrod::color::LIGHT_GREY)
                    .border(0.0)
                    .w_h(770.0, 320.0)
                    .mid_bottom_with_margin_on(ids.canvas, 100.0)
                    .set(ids.bg_att_sel, ui);

                // Only show stuff when there is a Pokemon selected
                if let Some(_) = app.sel_pkmn.clone().0 {
                    let techniques = app.techs.clone().unwrap();
                    let num_items = techniques.len();
                    let item_h = 64.0;

                    // List with all possible attacks for the selected Pokemon
                    let (mut events, scrollbar) = widget::ListSelect::single(num_items, item_h)
                        .scrollbar_next_to()
                        .w_h(200.0, 320.0)
                        .mid_left_with_margin_on(ids.bg_att_sel, 0.0)
                        // .x_y(-285.0, 100.0)
                        // .middle_of(ids.bg_att_sel)
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
                                    .border_color(conrod::color::LIGHT_GREY)
                                    .color(conrod::color::WHITE)
                                    .label(label)
                                    .label_color(app.label_color);
                                item.set(button, ui);
                            }

                            // Add attack to list when pressed
                            Event::Selection(selection) => {
                                println!("selected index (Attack): {:?}", selection);

                                if app.pkmn_moves.len() < 4 {
                                    app.pkmn_moves.push(techniques[selection].clone())
                                } else {
                                    println!("Error: Pokemon can only have 4 moves");
                                }
                            }
                            _ => {}
                        }
                    }

                    // Add buttons for attacks
                    let label1 = if app.pkmn_moves.len() > 0 {
                        app.pkmn_moves[0].get_name().to_string()
                    } else {
                        "".to_string()
                    };
                    let label2 = if app.pkmn_moves.len() > 1 {
                        app.pkmn_moves[1].get_name().to_string()
                    } else {
                        "".to_string()
                    };
                    let label3 = if app.pkmn_moves.len() > 2 {
                        app.pkmn_moves[2].get_name().to_string()
                    } else {
                        "".to_string()
                    };
                    let label4 = if app.pkmn_moves.len() > 3 {
                        app.pkmn_moves[3].get_name().to_string()
                    } else {
                        "".to_string()
                    };

                    if widget::Button::new()
                        .border(4.0)
                        .border_color(conrod::color::LIGHT_GREY)
                        .color(app.bg_color)
                        .label(&label1)
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
                        .border_color(conrod::color::LIGHT_GREY)
                        .color(app.bg_color)
                        .label(&label2)
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
                        .border_color(conrod::color::LIGHT_GREY)
                        .color(app.bg_color)
                        .label(&label3)
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
                        .border_color(conrod::color::LIGHT_GREY)
                        .color(app.bg_color)
                        .label(&label4)
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

                // Button to add selected Pokemon to team
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Select")
                    .label_color(app.label_color)
                    .left_from(ids.button_fight, 75.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_select, ui)
                    .was_clicked() {
                    println!("Select");

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
                    };
                }

                // Button to remove selected Pokemon from team
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
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
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Back")
                    .label_color(app.label_color)
                    .bottom_left_with_margins_on(ids.canvas, 35.0, 255.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_back, ui)
                    .was_clicked() {
                    println!("Back");
                    app.screen = Screen::Play;
                }

                // Button to start the fight
                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Fight")
                    .label_color(app.label_color)
                    .bottom_right_with_margins_on(ids.canvas, 35.0, 255.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_fight, ui)
                    .was_clicked() {
                    // temporaryly goes back to title screen
                    println!("Fight");
                    app.screen = Screen::BattleStart;
                }
            }

            // Draws Fight Screen
            if let Screen::BattleStart = app.screen {
                // Text BG
                widget::Canvas::new()
                    .color(conrod::color::LIGHT_ORANGE)
                    .border(2.0)
                    .w_h(WIDTH as f64, 240.0)
                    .mid_bottom_with_margin_on(ids.canvas, 0.0)
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
                    .color(conrod::color::LIGHT_RED)
                    .border(0.0)
                    .w_h(300.0, 350.0)
                    .bottom_right_with_margins_on(ids.canvas, 250.0, 10.0)
                    .set(ids.bg_sprite2, ui);

                // BG What to do next
                widget::Canvas::new()
                    .color(conrod::color::TRANSPARENT)
                    .border(0.0)
                    .w_h(350.0, 240.0)
                    .mid_right_of(ids.bg_text)
                    .set(ids.bg_whatdo, ui);

                widget::Tabs::new(&[(ids.tab_pokemon, "Pokémon"), (ids.tab_fight, "Fight")])
                    .w_h(200.0, 240.0)
                    .starting_canvas(ids.tab_fight)
                    .border(3.0)
                    .border_color(conrod::color::DARK_GREY)
                    .color(conrod::color::LIGHT_GREY)
                    .label_color(app.label_color)
                    .layout_vertically()
                    .bar_thickness(350.0)
                    .pad_top(-120.0)
                    .pad_bottom(120.0)
                    .pad_left(1280.0)
                    .pad_right(200.0)
                    .x_y(390.0, -360.0)
                    .set(ids.tab_whatdo, ui);

                if widget::Button::new()
                    .border(2.0)
                    .color(app.bg_color)
                    .label("Att1")
                    .label_color(app.label_color)
                    .top_left_with_margins_on(ids.tab_fight, 120.0, -815.0)
                    .w_h(465.0, 120.0)
                    .set(ids.button_att1, ui)
                    .was_clicked() {
                    println!("Att Button 1");
                }

                if widget::Button::new()
                    .border(2.0)
                    .color(app.bg_color)
                    .label("Att2")
                    .label_color(app.label_color)
                    .right_from(ids.button_att1, 0.0)
                    .w_h(465.0, 120.0)
                    .set(ids.button_att2, ui)
                    .was_clicked() {
                    println!("Att Button 2");
                }

                if widget::Button::new()
                    .border(2.0)
                    .color(app.bg_color)
                    .label("Att3")
                    .label_color(app.label_color)
                    .down_from(ids.button_att1, 0.0)
                    .w_h(465.0, 120.0)
                    .set(ids.button_att3, ui)
                    .was_clicked() {
                    println!("Att Button 3");
                }

                if widget::Button::new()
                    .border(2.0)
                    .color(app.bg_color)
                    .label("Att4")
                    .label_color(app.label_color)
                    .right_from(ids.button_att3, 0.0)
                    .w_h(465.0, 120.0)
                    .set(ids.button_att4, ui)
                    .was_clicked() {
                    println!("Att Button 4");
                }
            }
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
        text_test,

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
        button_att1,
        button_att2,
        button_att3,
        button_att4,
        button_swap,
    }
}
