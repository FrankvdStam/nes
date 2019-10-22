use glium::{glutin, Surface};
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use std::cell::Ref;
use std::rc::Rc;
use glium::backend::glutin::glutin::{PossiblyCurrent, Event, ContextWrapper};

#[derive(Debug, Copy, Clone)]
pub enum DebugEvent
{
    SetPixel { x: u32, y: u32, color: [f32; 3] },
    CloseWindow
}

#[derive(Copy, Clone)]
struct DebugWindowState
{
    x: u32,
    y: u32,
    color: [f32; 3],
}


pub struct DebugWindow
{
    imgui: imgui::Context,
    display: glium::Display,
    platform: WinitPlatform,
    renderer: Renderer,
    events_loop: glium::glutin::EventsLoop,

    //Holds state of the gui
    state: DebugWindowState,
}

impl DebugWindow
{
    pub fn new() -> Self
    {
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window_builder = glium::glutin::WindowBuilder::new();
        let context_builder = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

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

        DebugWindow
        {
            imgui,
            display,
            renderer,
            platform,
            events_loop,

            state: DebugWindowState
            {
                x: 0,
                y: 0,
                color: [0.0,0.0,0.0],
            }
        }
    }

    pub fn update(&mut self)
    {
        let gl_window = self.display.gl_window();
        let window = gl_window.window();
        let platform = &mut self.platform;
        let io_mut = self.imgui.io_mut();

        self.events_loop.poll_events(|event|
        {
            platform.handle_event(io_mut, &window, &event);
        });
    }

    pub fn render(&mut self) -> Option<DebugEvent>
    {
        let mut event: Option<DebugEvent> = None;

        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        let gl_window = self.display.gl_window();
        let window = gl_window.window();

        let io = self.imgui.io_mut();
        self.platform.prepare_frame(io, &window).expect("Failed to start frame");
        let mut ui = self.imgui.frame();
        let mut state = self.state.clone();

        Window::new(im_str!("Hello world"))
            .size([230.0, 800.0], Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .build(&ui, || {
                let mut demo = true;
                ui.show_demo_window(&mut demo);

                Slider::new(im_str!("x"), (0..=9)).build(&ui, &mut state.x);
                Slider::new(im_str!("y"), (0..=9)).build(&ui, &mut state.y);
                ColorPicker::new(im_str!("Color"), &mut state.color).build(&ui);
                if ui.button(im_str!("Set"), [100.0,20.0])
                {
                    event = Some(DebugEvent::SetPixel{ x: state.x, y: state.y, color: state.color});
                }

            });

        self.state = state;

        self.platform.prepare_render(&ui, &window);
        let draw_data = ui.render();
        self.renderer.render(&mut frame, draw_data).expect("Rendering failed");
        frame.finish().expect("Failed to swap buffers");

        return event;
    }
}