#[allow(dead_code)]
#[allow(unused_imports)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;
use graphics::types::Color;

use std::process;


const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];




pub const NES_SCREEN_WIDTH: u32 = 256;
pub const NES_SCREEN_HEIGHT: u32 = 240;
const SCREEN_BUFFER_SIZE: usize = (NES_SCREEN_WIDTH * NES_SCREEN_HEIGHT) as usize;




pub struct ScreenBuffer
{
	//Pixel buffer (using vector instead of array because array is created on the stack, causes stackoverflow.)
	colors: Vec<Color>,
}

impl ScreenBuffer
{
	pub fn new() -> ScreenBuffer
	{
		ScreenBuffer
		{
			colors: vec![BLACK; SCREEN_BUFFER_SIZE],
		}
	}

	pub fn set(&mut self, x: u32, y: u32, color: [f32; 4])
	{
		let index: usize = (x * NES_SCREEN_HEIGHT + y) as usize;		
		if index >= SCREEN_BUFFER_SIZE
		{
			panic!("Array out of bounds while setting screenbuffer.");
		}
		
		self.colors[index] = color;
	}

	pub fn get(&self, x: u32, y: u32) -> [f32; 4]
	{
		let index: usize = (x * NES_SCREEN_HEIGHT + y) as usize;		
		if index >= SCREEN_BUFFER_SIZE
		{
			panic!("Array out of bounds while getting screenbuffer.");
		}
		
		return self.colors[index];
	}
}






pub struct App {
	//Rendering related
    gl: GlGraphics,
	window: Window,
	events: Events,
	
	screen_buffer: ScreenBuffer,
	pixel_size: u32,
}

impl App {
	pub fn new(width: u32, height: u32, title: &'static str) -> Self
	{
		let open_gl_version = OpenGL::V3_3;

		let window: Window = WindowSettings::new(
            title,
            [width, height]
        )
        .graphics_api(open_gl_version)
		.exit_on_esc(true)
        .build()
        .unwrap();
		

		App
		{
			gl: GlGraphics::new(open_gl_version),
			events: Events::new(EventSettings::new()),
			window: window,

			screen_buffer: ScreenBuffer::new(),
			pixel_size: 1,
		}
	}

    pub fn render(&mut self, args: &RenderArgs) {
        

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		
        //let (x, y) = (args.window_size[0] / 2.0,
        //              args.window_size[1] / 2.0);
		
		let pixel_size = self.pixel_size;
		let screen_buffer = &self.screen_buffer;

        self.gl.draw(args.viewport(), |c, gl| 
		{
            clear(BLACK, gl);

			//Leaving this here, might be usefull later when transforms become a thing
            //let transform = c.transform.trans(x, y)
            //                           .rot_rad(rotation)
            //                           .trans(-25.0, -25.0);

			for x in 0..NES_SCREEN_WIDTH
			{
				for y in 0..NES_SCREEN_HEIGHT
				{
					//rectangle(RED, square, transform, gl);
					rectangle(screen_buffer.get(x, y), rectangle::square((x*pixel_size) as f64, (y*pixel_size) as f64, pixel_size as f64), c.transform, gl);
				}	
			}
        });
    }

	//This is a bit nasty. Gota fix it up..
	pub fn run(&mut self)
	{
		match self.events.next(&mut self.window)
		{
			Some(e) => {
				match e.render_args()
				{
					Some(r) => {
						self.render(&r);	
					}
					_ => {}
				}				

				match e.update_args()
				{
					Some(u) => {
						self.update(&u);	
					}
					_ => {}
				}
			}
			_ => {
				//TODO: create clean exit
				process::exit(1);
			}
		}	
	}

    pub fn update(&mut self, args: &UpdateArgs) {
        //dt contains time variables. Might be useful later.
        // self.rotation += 2.0 * args.dt;
    }

	pub fn set_pixel(&mut self, x: u32, y: u32, color: [f32; 4])
	{
		self.screen_buffer.set(x, y, color);
	}
}