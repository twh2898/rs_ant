
#[macro_use]
extern crate glium;

mod support;
mod world;
mod tile;

use world::World;
use support::Vertex;

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(1024, 760)
        .with_title("Ant".to_string())
        .build_glium()
        .expect("Couldnot create display!");

    let program = support::shaders::load_program(&display).unwrap();

    let vertex1 = Vertex::new(-1., -1., [1., 0., 0.]);
    let vertex2 = Vertex::new(0.0, 0.5, [0., 1., 0.]);
    let vertex3 = Vertex::new(0.5, -0.25, [0., 0., 1.]);
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut world = World::new();
    world.generate();

    let steptime = 1000.;
    let mut elapse = 0.;

    support::start_loop(|delta| {
        use glium::Surface;
        use glium::glutin::Event;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        elapse += delta;

        if elapse > steptime {
            world.step();
            elapse = 0.;
        }

        target
            .draw(&vertex_buffer,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default())
            .unwrap();

        target.finish().expect("Could not finish! ;)");

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return support::Action::Stop,
                _ => (),
            }
        }

        support::Action::Continue
    });
}
