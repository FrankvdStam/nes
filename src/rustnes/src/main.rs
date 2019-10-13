#[allow(dead_code)]
#[allow(non_snake_case)]

#[macro_use]
extern crate glium;
extern crate rand;
extern crate imgui;
//extern crate takeable_option;


use glium::{glutin, Surface};
//use rand::{RngCore, Rng};
use std::ops;

pub mod graphics;
use graphics::*;


mod cpu;
mod bus;
mod memory;
use cpu::Cpu;
use bus::Bus;

use std::cell::RefCell;
use std::fs::read_to_string;

fn main2()
{
	let mut i:i32 = 1;
    let ref_i = &mut i;
	*ref_i = 1;
    let another_ref_i = &mut i;
	*another_ref_i = 1;

	//let mut cpu = Cpu::new();
	println!("Hello, world!");

   //opengl_example();
}



fn main() {


    //glium initialization
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window_builder = glium::glutin::WindowBuilder::new();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
    //let display2 = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();


    //imgui initialization
    //let mut imgui = Context::create();
    //imgui.set_ini_filename(None);
    //let mut platform = WinitPlatform::init(&mut imgui);
    //{
    //    let gl_window = display.gl_window();
    //    let window = gl_window.window();
    //    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    //}
    //imgui.io_mut().font_global_scale = (1.0 / platform.hidpi_factor()) as f32;
    //let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    //let gl_window = display.gl_window();
    //let window = gl_window.window();

    let mut canvas = Canvas::new(&display, 256, 240, 2.0, 2.0, -1.0, 1.0);
    let mut rng = rand::thread_rng();

    let mut closed = false;


    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut color: [f32; 3] = [0.0, 0.0, 0.0];

    let mut debug_window = DebugWindow::new(&events_loop);

    //let window = debug_window.diplay.gl_window().window();

    while !closed {
        events_loop.poll_events(|event| {
            //platform.handle_event(imgui.io_mut(), &window, &event);
            debug_window.handle_events(&event);

            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        canvas.draw(&mut frame);

        println!("Borrow frame");
        debug_window.render(&mut frame);
        println!("end Borrow frame");

        frame.finish().expect("Failed to swap buffers");

    }
}