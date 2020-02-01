#[allow(dead_code)]
#[allow(non_snake_case)]

#[macro_use]
extern crate glium;
extern crate rand;
extern crate imgui;

use glium::{glutin, Surface};
use std::ops;

pub mod graphics;
use graphics::*;

pub mod hardware;
pub use hardware::*;

use std::cell::RefCell;
use std::fs::read_to_string;

fn main()
{
    let mut rng = rand::thread_rng();

    let mut nes_window = NesWindow::new(256, 240, 2.0, 2.0, -1.0, 1.0);
    let mut debug_window = DebugWindow::new();

    let mut running = true;
    let mut display_debug_window = true;

    while running
    {
        let mut debug_event: Option<DebugEvent> = None;

        if display_debug_window
        {
            debug_window.update();
            debug_event = debug_window.render();

            match debug_event
            {
                Some(event) =>
                {
                    match event
                    {
                        DebugEvent::CloseWindow => display_debug_window = false,
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        nes_window.update(debug_event);
        nes_window.render();
    }
}