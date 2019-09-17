#[allow(dead_code)]
#[allow(non_snake_case)]

extern crate glfw;

mod cpu;
mod bus;
mod memory;


use glfw::{Action, Context, Key};
use cpu::Cpu;
use bus::Bus;

use std::cell::RefCell;


fn main()
{

	let mut i:i32 = 1;
    let ref_i = &mut i;
	*ref_i = 1;
    let another_ref_i = &mut i;
	*another_ref_i = 1;

	//let mut cpu = Cpu::new();
	println!("Hello, world!");
}


fn opengl_example() {
   let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}