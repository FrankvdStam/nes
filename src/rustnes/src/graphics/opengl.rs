use glium::Display;
use glium::Program;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

//Get simple shaders with color and position
pub fn get_program(display: &Display) -> Program
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