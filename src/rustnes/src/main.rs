//#[allow(dead_code)]
//#[allow(non_snake_case)]
//
//extern crate glfw;
//
//mod cpu;
//mod bus;
//mod memory;
//
//
//use glfw::{Action, Context, Key};
//use cpu::Cpu;
//use bus::Bus;
//
//use std::cell::RefCell;
//
//
//fn main12()
//{
//
//	let mut i:i32 = 1;
//    let ref_i = &mut i;
//	*ref_i = 1;
//    let another_ref_i = &mut i;
//	*another_ref_i = 1;
//
//	//let mut cpu = Cpu::new();
//	println!("Hello, world!");
//}
//
//
//fn opengl_example() {
//   let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//
//    // Create a windowed mode window and its OpenGL context
//    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
//        .expect("Failed to create GLFW window.");
//
//    // Make the window's context current
//    window.make_current();
//    window.set_key_polling(true);
//
//    // Loop until the user closes the window
//    while !window.should_close() {
//        // Swap front and back buffers
//        window.swap_buffers();
//
//        // Poll for and process events
//        glfw.poll_events();
//        for (_, event) in glfw::flush_messages(&events) {
//            println!("{:?}", event);
//            match event {
//                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
//                    window.set_should_close(true)
//                },
//                _ => {},
//            }
//        }
//    }
//}




extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;

use app::App;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    //let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    //let mut window: Window = WindowSettings::new(
    //        "spinning-square",
    //        [200, 200]
    //    )
    //    .graphics_api(opengl)
    //    .exit_on_esc(true)
    //    .build()
    //    .unwrap();

    // Create a new game and run it.
    let mut app = App::new(200, 200, "spinning-square");

	while true 
	{
		app.run_events();
	}
    //let mut events = Events::new(EventSettings::new());
    //while let Some(e) = events.next(&mut window) {
    //    if let Some(r) = e.render_args() {
    //        app.render(&r);
    //    }
	//
    //    if let Some(u) = e.update_args() {
    //        app.update(&u);
    //    }
    //}
}