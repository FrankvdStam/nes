use glium::{glutin, Surface};
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};


use std::cell::Ref;
use std::rc::Rc;
use glium::backend::glutin::glutin::{PossiblyCurrent, Event, ContextWrapper};
//pub extern crate glium::takeable_option;
//pub use glium::takeable_option;

//pub use glium::takeable_option as glium_takeable_option;

pub struct DebugWindow
{
    pub imgui: imgui::Context,
    pub display: glium::Display,
    platform: WinitPlatform,
    renderer: Renderer,

    //gl_window: Ref<&'static Takeable<ContextWrapper<PossiblyCurrent, Window>>>,
    //gl_window:  Ref<&'static u16>,
    //window: &'static glutin::Window,

    x: i32,
    y: i32,
    color: [f32; 3],
}

impl DebugWindow
{
    pub fn new(events_loop: &glium::glutin::EventsLoop) -> Self
    {
        let b2: Ref<u32>;


        let window_builder = glium::glutin::WindowBuilder::new();
        let context_builder = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context_builder, events_loop).unwrap();

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

        //Ref<'_, takeable_option::Takeable<glium::glutin::ContextWrapper<glium::glutin::PossiblyCurrent, glium::glutin::Window>>>
        //let gl_window: Ref<glium_takeable_option::Takeable<ContextWrapper<PossiblyCurrent, Window>>> = display.gl_window();
        //let window = gl_window.window();

        DebugWindow
        {
            imgui,
            display,
            renderer,
            platform,

            //gl_window,
            //window,

            x: 0,
            y: 0,
            color: [0.0, 0.0, 0.0],
        }
    }

    pub fn handle_events(&mut self, event: &Event)
    {
        self.platform.handle_event(self.imgui.io_mut(), self.display.gl_window().window(), &event);
    }

    pub fn render(&mut self, frame: &mut glium::Frame)
    {
        let gl_window = self.display.gl_window();
        let window = gl_window.window();

        let io = self.imgui.io_mut();
        self.platform.prepare_frame(io, &window).expect("Failed to start frame");
        let mut ui = self.imgui.frame();

        let mut x = self.x.clone();
        let mut y = self.y.clone();
        let mut color = self.color.clone();



        Window::new(im_str!("Hello world"))
            .size([230.0, 800.0], Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .build(&ui, || {
                let mut demo = true;
                ui.show_demo_window(&mut demo);
                Slider::new(im_str!("x"), (0..=9)).build(&ui, &mut x);
                Slider::new(im_str!("y"), (0..=9)).build(&ui, &mut y);
                ColorPicker::new(im_str!("Color"), &mut color).build(&ui);
                if ui.button(im_str!("Set"), [100.0,20.0])
                {
                    //canvas.set_color(x as u32, y as u32, color);
                    //canvas.refresh_vertex_buffer(&display);
                }
            });

        self.x = x;
        self.y = y;
        self.color = color;

        self.platform.prepare_render(&ui, &window);

        let draw_data = ui.render();
        println!("rendering");
        self.renderer.render(frame, draw_data).expect("Rendering failed");
    }

    //pub fn window(&self) -> &glium::glutin::Window
    //{
    //    let thing1: Ref<Takeable<glutin::WindowedContext<Pc>>> = self.display.gl_window();
    //    let thing: &'static glium::glutin::Window = self.display.gl_window().window();
    //    return thing;
    //}
}