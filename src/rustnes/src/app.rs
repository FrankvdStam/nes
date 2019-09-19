extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

/*
 let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
*/

pub struct App {
	//Rendering related
    gl: GlGraphics,
	window: Window,
	events: Events,

    rotation: f64
}

impl App {
	pub fn new(width: u32, height: u32, title: &'static str) -> Self
	{
		let openGlVersion = OpenGL::V3_3;

		let mut window: Window = WindowSettings::new(
            title,
            [width, height]
        )
        .graphics_api(openGlVersion)
		.exit_on_esc(true)
        .build()
        .unwrap();
		
		App
		{
			gl: GlGraphics::new(openGlVersion),
			events: Events::new(EventSettings::new()),
			window: window,
			rotation: 0.0
		}
	}

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
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
			_ => {}
		}	
	}

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}