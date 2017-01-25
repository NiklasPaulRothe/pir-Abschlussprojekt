extern crate find_folder;
extern crate conrod;

use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;

pub fn draw_startscreen() {

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

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

    // List of entries to display.
    let list_items = [
        "Play".to_string(),
        "Options".to_string(),
        "Exit".to_string(),
    ];

    // List of selections. Will be updated by the widget.
    let list_selected: ::std::collections::HashSet<usize> = ::std::collections::HashSet::new();

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

            widget::Canvas::new().color(conrod::color::WHITE).set(ids.canvas, ui);

            // Instantiate the `ListSelect` widget.
            let num_items = list_items.len();
            let item_h = 32.0;
            let (mut events, _) = widget::ListSelect::single(num_items, item_h)
                .w_h(350.0, 100.0)
                .middle_of(ids.canvas)
                .set(ids.list_select, ui);

            // Handle the `ListSelect`s events.
            while let Some(event) = events.next(ui, |i| list_selected.contains(&i)) {
                use conrod::widget::list_select::Event;
                match event {

                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let label = &list_items[item.i];
                        let font_size = item_h as conrod::FontSize / 2;
                        let (color, label_color) = match list_selected.contains(&item.i) {
                            true => (conrod::color::LIGHT_GREY, conrod::color::BLACK),
                            false => (conrod::color::WHITE, conrod::color::BLACK),
                        };
                        let button = widget::Button::new()
                            .border(1.0)
                            .color(color)
                            .label(label)
                            .label_font_size(font_size)
                            .label_color(label_color);
                        item.set(button, ui);
                    },

                    _ => {}
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
