extern crate find_folder;
extern crate conrod;

use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;

pub fn draw_startscreen() {

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const BUTTON_W: f64 = 100.0;
    const BUTTON_H: f64 = 30.0;

    // Construct the window.
    let mut window: Window =
        piston::window::WindowSettings::new("PokemonBattleArena", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .vsync(true)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Create the event loop.
    let mut events = WindowEvents::new();

    // Construct the `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Create Ids for every used widget
    widget_ids! {
         struct Ids {
            canvas,
            list_select,
            button_play,
            button_options,
            button_exit,
        }
    }
    let ids = Ids::new(ui.widget_id_generator());

    // Add a font from file
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/arial/arial.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // No text to draw -> create an empty text texture cache.
    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in this case none)
    let image_map = conrod::image::Map::new();

    // Poll events from the window.
    while let Some(event) = window.next_event(&mut events) {

        // Convert the piston event to a conrod event.
        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

            // Instantiate the conrod widgets.
            let ui = &mut ui.set_widgets();

            // Create new empty canvas
            widget::Canvas::new().color(conrod::color::WHITE).set(ids.canvas, ui);

            // Play button
            if widget::Button::new()
                .border(1.0)
                .color(conrod::color::WHITE)
                .label("Play")
                .label_color(conrod::color::BLACK)
                .middle_of(ids.canvas)
                .w_h(BUTTON_W, BUTTON_H)
                .set(ids.button_play, ui)
                .was_clicked()
            {
                // Right now only prints play
                // Todo: draw new window with different play modi
                println!("Play");
            }

            // Options button
            if widget::Button::new()
                .border(1.0)
                .color(conrod::color::WHITE)
                .label("Options")
                .label_color(conrod::color::BLACK)
                .down_from(ids.button_play, 0.0)
                .w_h(BUTTON_W, BUTTON_H)
                .set(ids.button_options, ui)
                .was_clicked()
            {
                // Right now only prints option
                // Todo: draw new window with options menu
                println!("Options");
            }

            // Exit button
            if widget::Button::new()
                .border(1.0)
                .color(conrod::color::WHITE)
                .label("Exit")
                .label_color(conrod::color::BLACK)
                .down_from(ids.button_options, 0.0)
                .w_h(BUTTON_W, BUTTON_H)
                .set(ids.button_exit, ui)
                .was_clicked()
            {
                // exits the programm
                ::std::process::exit(0);
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
