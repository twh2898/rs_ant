
extern crate glium;

pub fn load_program<'a, D>(display: &'a D) -> Result<glium::Program, glium::ProgramCreationError>
    where D: glium::backend::Facade
{
    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
}

pub const vertex_shader_src: &'static str = r#"
     #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

pub const fragment_shader_src: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
