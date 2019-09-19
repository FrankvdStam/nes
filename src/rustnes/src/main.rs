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




//extern crate piston;
//extern crate graphics;
//extern crate glutin_window;
//extern crate opengl_graphics;
//
//use piston::window::WindowSettings;
//use piston::event_loop::*;
//use piston::input::*;
//use glutin_window::GlutinWindow as Window;
//use opengl_graphics::{ GlGraphics, OpenGL };
//use std::boxed::Box;
//use std::borrow::BorrowMut;

//extern crate rand;
//
//mod app;
//
//use rand::Rng;
//use app::App;
//
//fn main() {
//	
//	let mut rng = rand::thread_rng();
//	
//	//let num = rng.gen_range::<u32>(0, 10);
//    
//	
//    let mut app = App::new(800, 600, "spinning-square");
//
//	loop
//	{
//		if rng.gen_range(0, 10) > 7
//		{
//			let x = rng.gen_range(0, app::NES_SCREEN_WIDTH);
//			let y = rng.gen_range(0, app::NES_SCREEN_HEIGHT);
//			let color: [f32; 4] = 
//			[
//				rng.gen::<f32>(),
//				rng.gen::<f32>(),
//				rng.gen::<f32>(),
//				1.0
//			];
//			app.set_pixel(x, y, color);
//		}
//
//		
//
//		//println!("Integer: {}", rng.gen_range(0, 10));
//
//		app.run();
//
//	}
//}


extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

		if window.is_key_down(Key::Space)
		{
			for i in buffer.iter_mut() {
				*i = 0x00_00_00_ff; // write something more funny here!
				//Notation: 0xalpha_r_g_b
			}
		}

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}