use rand::prelude::*;
use glium::index::PrimitiveType;
use glium::{Surface, VertexBuffer, Display, Frame, Program};
use glium::backend::Facade;
use glium::buffer::BufferMode;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

//Get simple shaders with color and position
fn get_program(display: &Display) -> Program
{
    let vertex_shader_src = r#"
    #version 140

	out vec3 mcolor;
	in vec3 color;

    in vec2 position;

    void main() {
		mcolor = color;
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

	in vec3 mcolor;
    out vec4 Vcolor;

    void main() {
        Vcolor = vec4(mcolor, 1.0);
    }
"#;

    return glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
}


//Generates rectangles
fn generate_vertices_and_indices(buffer_width: u32, buffer_height: u32, screen_width: f32, screen_height: f32, screen_x: f32, screen_y: f32) -> (Vec<Vertex>, Vec<u32>)
{
    let mut vertices: Vec<Vertex> =  Vec::with_capacity((buffer_width*buffer_height*4) as usize);
    let mut indices: Vec<u32> =  Vec::with_capacity((buffer_width*buffer_height*6) as usize);

    let rect_size = f32::min(screen_width / buffer_width as f32, screen_height / buffer_height as f32);
    let mut rng = rand::thread_rng();

    for x in 0..buffer_width
    {
        for y in 0..buffer_height
        {
            let fx = screen_x + (x as f32 * rect_size) ;
            let fy = screen_y - (y as f32 * rect_size) ;

            let color: [f32; 3] = [rng.gen(), rng.gen(), rng.gen()];

            let start_index: u32 = vertices.len() as u32;

            //Up left
            vertices.push(Vertex { position: [  fx          ,   fy          ], color } );
            //Up right
            vertices.push(Vertex { position: [  fx+rect_size,   fy          ], color } );
            //down left
            vertices.push(Vertex { position: [  fx          ,   fy-rect_size], color } );
            //down right
            vertices.push(Vertex { position: [  fx+rect_size,   fy-rect_size], color } );


            //first triangle is 0, 1, 2. Equates to start_index, start_index+1, start_index+2.
            //Second triangle is 3, 1, 2. Equates to start_index+3, start_index+1, start_index+2.
            //Next cycle, the start index will be 4.
            //first triangle is 4, 5, 6
            //second triangle is 7, 5, 6
            indices.push(start_index);
            indices.push(start_index+1);
            indices.push(start_index+2);
            indices.push(start_index+3);
            indices.push(start_index+1);
            indices.push(start_index+2);
        }
    }

    println!("usize max {}", std::usize::MAX);
    println!("{} vertices and {} indices", vertices.len(), indices.len());
    return (vertices, indices);
}



//Holds all the data to render NES pixels, with a nice wrapper to set colors so that you don't have to dig into vertices manually.
#[derive(Debug)]
pub struct Canvas
{
    pub buffer_width:  u32,
    pub buffer_height: u32,

    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    program: Program,
}


impl Canvas
{
    pub fn new(display: &Display, buffer_width: u32, buffer_height: u32, screen_width: f32, screen_height: f32, screen_x: f32, screen_y: f32) -> Self
    {
        let (vertices, indices) = generate_vertices_and_indices(buffer_width, buffer_height, screen_width, screen_height, screen_x, screen_y);

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display,PrimitiveType::TrianglesList, &indices).unwrap();
        let program = get_program(display);

        Canvas
            {
                buffer_width,
                buffer_height,

                vertices,
                indices,

                vertex_buffer,
                index_buffer,
                program,
            }
    }

    pub fn draw(&self, frame: & mut Frame)
    {
        frame.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &glium::uniforms::EmptyUniforms,&Default::default()).unwrap();
    }

    pub fn set_color(& mut self, x: u32, y: u32, color: [f32; 3])
    {
        //4 vertices per nes pixel, means we need to multiply by 4 and then set all 4 pixels.
        let index: usize = ((x* 4) * self.buffer_height + (y*4)) as usize;
        for i in index..index+4
            {
                self.vertices[i].color = color;
            }
    }

    pub fn refresh_vertex_buffer(&mut self, display: &Display)
    {
        self.vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();
    }
}