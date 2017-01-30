extern crate find_folder;
extern crate conrod;

use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;
use db;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BUTTON_W: f64 = 150.0;
const BUTTON_H: f64 = 30.0;

enum Screen {
    Title,
    Play,
    Options,
    PokemonChoose,
}

struct App {
    screen: Screen,
    label_color: conrod::Color,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::Title,
            label_color: conrod::color::BLACK,
        }
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
    let font_path = assets.join("fonts/arial/arial.ttf");
    ui.fonts.insert_from_file(font_path).unwrap_or_else(
        |e| { panic!("Failed to get font: {}", e) }
    );

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
            let mut ui = ui.set_widgets();
            set_ui(&mut ui, &mut ids, &mut app)
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
        button_play_back,
        button_up,
        button_down,
        button_choose,
    }
}

fn set_ui(ui: &mut conrod::UiCell, ids: &mut Ids, app: &mut App) {
    use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

    // Create new empty canvas
    widget::Canvas::new().color(conrod::color::WHITE).set(ids.canvas, ui);

    // draws Title-Screen
    if let Screen::Title = app.screen {
        // Play button
        // Shows Play-Screen when clicked
        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
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
        // TODO: draw new window with options menu
        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
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
            .color(conrod::color::WHITE)
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
        // not implemented yet
        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
            .label("Singleplayer")
            .label_color(app.label_color)
            .middle_of(ids.canvas)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_sp, ui)
            .was_clicked()
        {
            app.screen = Screen::PokemonChoose;
            println!("Singleplayer");
        }

        // Multiplayer button
        // not implemented yet
        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
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
            .color(conrod::color::WHITE)
            .label("Back")
            .label_color(app.label_color)
            .down_from(ids.button_mp, 0.0)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_play_back, ui)
            .was_clicked()
        {
            println!("Back");
            app.screen = Screen::Title;
        }
    }

    //draws Options-Screen
    if let Screen::Options = app.screen {
        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
            .label("Back")
            .label_color(app.label_color)
            .down_from(ids.button_mp, 0.0)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_play_back, ui)
            .was_clicked()
        {
            println!("Back");
            app.screen = Screen::Title;
        }
    }

    // Pokemon choose Screen should be able to setup a vec with atleast one but a maximum of 6
    // pokemon for a player.
    let mut index = 0;
    if let Screen::PokemonChoose = app.screen {

        // let pokedex = db::pokedex::Pokedex::new();
        // let entries = pokedex.get_entries();
        let string1: String = index.to_string();
        let string: &str = &string1;

        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
            .label("<-")
            .label_color(app.label_color)
            .middle_of(ids.canvas)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_up, ui)
            .was_clicked()
        {
            println!("<-");
            index -= 1;
            app.screen = Screen::PokemonChoose;
        }

        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
            .label(string)
            .label_color(app.label_color)
            .down_from(ids.button_up, 0.0)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_choose, ui)
            .was_clicked()
        {
            println!("{}", index);
            app.screen = Screen::PokemonChoose;
        }

        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
            .label("->")
            .label_color(app.label_color)
            .down_from(ids.button_choose, 0.0)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_down, ui)
            .was_clicked()
        {
            println!("->");
            index += 1;
            app.screen = Screen::PokemonChoose;
        }

        if widget::Button::new()
            .border(1.0)
            .color(conrod::color::WHITE)
            .label("Back")
            .label_color(app.label_color)
            .down_from(ids.button_down, 0.0)
            .w_h(BUTTON_W, BUTTON_H)
            .set(ids.button_play_back, ui)
            .was_clicked()
        {
            println!("Back");
            app.screen = Screen::Title;
        }
    }
}
