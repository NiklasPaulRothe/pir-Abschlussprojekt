#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate conrod;
extern crate rustc_serialize;
extern crate time;


use time::get_time;

mod arena;
mod db;
mod graphic;
mod player;

fn main() {
    graphic::windows::draw_startscreen();
}
