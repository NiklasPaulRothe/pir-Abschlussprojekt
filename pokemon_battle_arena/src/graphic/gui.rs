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
    Battle,
}

/// App struct, which contains important data
///     screen:         screen that gets drawn
///     label_color:    color of labels (text)
///     bg_color:       background color
///     pokedex:        currently used pokedex
///     pkmn_team:      current team
///     sel_pkmn:       currently selected pokemon
#[derive(Clone)]
struct App {
    screen: Screen,
    label_color: conrod::Color,
    bg_color: conrod::Color,
    pokedex: db::pokedex::Pokedex,
    pkmn_team: Vec<db::pokemon_token::PokemonToken>,
    sel_pkmn: Option<db::pokemon_token::PokemonToken>,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::Title,
            label_color: conrod::color::BLACK,
            bg_color: conrod::color::WHITE,
            pokedex: db::pokedex::Pokedex::new(),
            pkmn_team: Vec::new(),
            sel_pkmn: None,
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
    let mut window: Window =
        piston::window::WindowSettings::new("PokemonBattleArena", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .vsync(true)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build window: {}", e) });

    // Create the event loop.
    let mut events = WindowEvents::new();

    // Construct the `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Create Ids for every used widget
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a font from file
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap_or_else(
        |e| { panic!("Failed to find folder: {}", e) }
    );
    let font_path = assets.join("fonts/calibri/calibri.ttf");
    ui.fonts.insert_from_file(font_path).unwrap_or_else(
        |e| { panic!("Failed to get font: {}", e) }
    );

    // No text to draw -> create an empty text texture cache.
    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in this case none)
    let image_map = conrod::image::Map::new();

    let mut app = App::new();

    // List of selections, should be same length as list of entries. Will be updated by the widget.
    let mut list_selected: ::std::collections::HashSet<usize> = ::std::collections::HashSet::new();

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
            widget::Canvas::new().color(app.bg_color).set(ids.canvas, ui);

            // draws Title-Screen
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
                    .was_clicked()
                {
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
                    .was_clicked()
                {
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
                    .was_clicked()
                {
                    ::std::process::exit(0);
                }
            }

            // draws Play-Screen
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
                    .was_clicked()
                {
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
                    .was_clicked()
                {
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
                    .was_clicked()
                {
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
                    .was_clicked()
                {
                    println!("Back");
                    app.screen = Screen::Title;
                }
            }

            // Draws Choose Team Screen
            // IDEAS:   - pictures of pokemon
            //          - filter gen/type (get pokedex with only specific type/gen)
            //          - search ( fn that gives Vec with all pokemon that have certain string in name)
            if let Screen::ChooseTeam = app.screen {
                let pokedex_entries = app.pokedex.get_entries();
                let num_items = pokedex_entries.len();
                let item_h = 32.0;
                let (mut events, scrollbar) = widget::ListSelect::single(num_items, item_h)
                    .scrollbar_next_to()
                    .w_h(200.0, 650.0)
                    .mid_left_with_margin_on(ids.canvas, 25.0)
                    .scrollbar_width(15.0)
                    .set(ids.s_list_pokemon, ui);

                let mut pkmn_names = Vec::new();
                for pokemon in pokedex_entries {
                    pkmn_names.push(pokemon.get_name());
                }

                // Handle the `ListSelect`s events.
                while let Some(event) = events.next(ui, |i| list_selected.contains(&i)) {
                    use conrod::widget::list_select::Event;
                    match event {
                        // For the `Item` events we instantiate the `List`'s items.
                        Event::Item(item) => {
                            let label = &pkmn_names[item.i];
                            let font_size = item_h as conrod::FontSize / 2;

                            let button = widget::Button::new()
                                .border(0.0)
                                .color(conrod::color::LIGHT_GREY)
                                .label(label)
                                .label_font_size(font_size)
                                .label_color(app.label_color);
                            item.set(button, ui);
                        },

                        // The selection has changed.
                        Event::Selection(selection) => {
                            println!("selected index: {:?}", selection);

                            if app.pkmn_team.len() < 6 {
                                app.pkmn_team.push(
                                    db::pokemon_token::PokemonToken::from_model(
                                        app.pokedex.pokemon_by_id(selection + 1).unwrap()
                                    )
                                );

                                println!();
                                println!("Current Team:");
                                for i in 0..app.pkmn_team.len() {
                                    println!("{:?}", app.pkmn_team[i].get_name());
                                }
                            } else {
                                println!("Error: only 6 pokemon per team allowed");
                            }
                        }

                        // The remaining events indicate interactions with the `ListSelect` widget.
                        _ => {},
                    }
                }

                // Instantiate the scrollbar for the list.
                if let Some(s) = scrollbar { s.set(ui); }

                // ***** Buttons mit ausgewählten Pokemon *****
                // TODO/IDEE : makro für schönere/kürzere implementierung
                if app.pkmn_team.len() > 0 {
                    let label_p1 = app.pkmn_team[0].get_name();
                    if widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label(&label_p1)
                        .label_color(app.label_color)
                        .top_right_with_margin_on(ids.canvas, 25.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p1, ui)
                        .was_clicked()
                    {
                        println!("p1");
                        app.pkmn_team.remove(0);
                    }
                } else {
                    widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label("-")
                        .label_color(app.label_color)
                        .top_right_with_margin_on(ids.canvas, 25.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p1, ui);
                }

                if app.pkmn_team.len() > 1 {
                    let label_p2 = app.pkmn_team[1].get_name();
                    if widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label(&label_p2)
                        .label_color(app.label_color)
                        .down_from(ids.button_p1, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p2, ui)
                        .was_clicked()
                    {
                        println!("p2");
                        app.pkmn_team.remove(1);
                    }
                } else {
                    widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label("-")
                        .label_color(app.label_color)
                        .down_from(ids.button_p1, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p2, ui);
                }

                if app.pkmn_team.len() > 2 {
                    let label_p3 = app.pkmn_team[2].get_name();
                    if widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label(&label_p3)
                        .label_color(app.label_color)
                        .down_from(ids.button_p2, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p3, ui)
                        .was_clicked()
                    {
                        println!("p3");
                        app.pkmn_team.remove(2);
                    }
                } else {
                    widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label("-")
                        .label_color(app.label_color)
                        .down_from(ids.button_p2, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p3, ui);
                }

                if app.pkmn_team.len() > 3 {
                    let label_p4 = app.pkmn_team[3].get_name();
                    if widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label(&label_p4)
                        .label_color(app.label_color)
                        .down_from(ids.button_p3, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p4, ui)
                        .was_clicked()
                    {
                        println!("p4");
                        app.pkmn_team.remove(3);
                    }
                } else {
                    widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label("-")
                        .label_color(app.label_color)
                        .down_from(ids.button_p3, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p4, ui);
                }

                if app.pkmn_team.len() > 4 {
                    let label_p5 = app.pkmn_team[4].get_name();
                    if widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label(&label_p5)
                        .label_color(app.label_color)
                        .down_from(ids.button_p4, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p5, ui)
                        .was_clicked()
                    {
                        println!("p5");
                        app.pkmn_team.remove(4);
                    }
                } else {
                    widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label("-")
                        .label_color(app.label_color)
                        .down_from(ids.button_p4, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p5, ui);
                }

                if app.pkmn_team.len() > 5 {
                    let label_p6 = app.pkmn_team[5].get_name();
                    if widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label(&label_p6)
                        .label_color(app.label_color)
                        .down_from(ids.button_p5, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p6, ui)
                        .was_clicked()
                    {
                        println!("p6");
                        app.pkmn_team.remove(5);
                    }
                } else {
                    widget::Button::new()
                        .border(1.0)
                        .color(app.bg_color)
                        .label("-")
                        .label_color(app.label_color)
                        .down_from(ids.button_p5, 0.0)
                        .w_h(BUTTON_W, 650.0/6.0)
                        .set(ids.button_p6, ui);
                }

                let stats = "TEST TEST TEST TEST TEST TEST TEST TEST TEST TEST TEST TEST ";

                widget::Text::new(stats)
                    .color(app.label_color)
                    .middle_of(ids.canvas)
                    .align_text_left()
                    .line_spacing(10.0)
                    .set(ids.text_sel_pkmn, ui);

                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("Back")
                    .label_color(app.label_color)
                    .mid_bottom_with_margin(5.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_back, ui)
                    .was_clicked()
                {
                    println!("Back");
                    app.screen = Screen::Title;
                }

                if widget::Button::new()
                    .border(1.0)
                    .color(app.bg_color)
                    .label("FIGHT")
                    .label_color(app.label_color)
                    .up_from(ids.button_back, 0.0)
                    .w_h(BUTTON_W, BUTTON_H)
                    .set(ids.button_fight, ui)
                    .was_clicked()
                {
                    // temporaryly goes back to title screen
                    println!("FIGHT");
                    app.screen = Screen::Title;
                }
            }
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                piston::window::draw(c, g, primitives,
                                     &mut text_texture_cache,
                                     &image_map,
                                     texture_from_image);
            }
        });
    }
}

widget_ids! {
    struct Ids {
        // *** canvas ***
        canvas,

        // *** buttons ***
        button_play,
        button_options,
        button_exit,
        button_sp,
        button_mp,
        button_back,
        button_p1,
        button_p2,
        button_p3,
        button_p4,
        button_p5,
        button_p6,
        button_fight,

        // *** selection_list ***
        s_list_pokemon,

        // *** text ***
        text_sel_pkmn,
    }
}
