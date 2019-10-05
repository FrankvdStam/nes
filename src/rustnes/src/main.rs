#[allow(dead_code)]
#[allow(non_snake_case)]

#[macro_use]
extern crate glium;
extern crate rand;
extern crate imgui;

use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use rand::{RngCore, Rng};
use std::ops;

pub mod graphics;
use graphics::Canvas;


mod cpu;
mod bus;
mod memory;
use cpu::Cpu;
use bus::Bus;

use std::cell::RefCell;

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
    use glium::{glutin, Surface};

    //glium initialization
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window_builder = glium::glutin::WindowBuilder::new();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    //imgui initialization
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    }
    imgui.io_mut().font_global_scale = (1.0 / platform.hidpi_factor()) as f32;
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let gl_window = display.gl_window();
    let window = gl_window.window();

    let mut canvas = Canvas::new(&display, 256, 240, 1.0, 1.0, -0.5, 0.5);
    let mut rng = rand::thread_rng();

    let mut closed = false;


    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut color: [f32; 3] = [0.0, 0.0, 0.0];

    while !closed {
        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &window, &event);

            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        canvas.draw(& mut target);

        //for i in 0..5
        //{
        //    let x = rng.gen_range(0, canvas.buffer_width);
        //    let y = rng.gen_range(0, canvas.buffer_height);
        //    canvas.set_color(x, y, [1.0, 1.0, 1.0]);
        //}
        //canvas.refresh_vertex_buffer(&display);

        let io = imgui.io_mut();
        platform.prepare_frame(io, &window).expect("Failed to start frame");
        let mut ui = imgui.frame();

        Window::new(im_str!("Hello world"))
            .size([300.0, 500.0], Condition::FirstUseEver)
            .build(&ui, || {
                let mut demo = true;
                ui.show_demo_window(&mut demo);

                Slider::new(im_str!("x"), (0..=9)).build(&ui, &mut x);
                Slider::new(im_str!("y"), (0..=9)).build(&ui, &mut y);
                ColorPicker::new(im_str!("Color"), &mut color).build(&ui);
                if ui.button(im_str!("Set"), [100.0,20.0])
                {
                    canvas.set_color(x as u32, y as u32, color);
                    canvas.refresh_vertex_buffer(&display);
                }
            });
        platform.prepare_render(&ui, &window);
        let draw_data = ui.render();
        renderer.render(&mut target, draw_data).expect("Rendering failed");
        /*
            last_frame = io.update_delta_time(last_frame);
            let mut ui = imgui.frame();
            run_ui(&mut run, &mut ui);

            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(&ui, &window);
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");

        */
        target.finish().expect("Failed to swap buffers");
    }
}