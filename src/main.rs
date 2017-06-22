
#[macro_use]
extern crate glium;

mod support;
mod world;
mod tile;
mod grid;

use world::World;
use support::Vertex;
use support::Drawable;

fn main() {
    use glium::DisplayBuild;

    // Create Window

    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(1024, 760)
        .with_title("Ant".to_string())
        .build_glium()
        .expect("Couldnot create display!");

    // Load Shader Program

    let program = support::shaders::load_program(&display).unwrap();

    // Load Vertex Buffers

    let grid = grid::Grid::new()
        .with_cols(160)
        .with_rows(100)
        .with_border(false)
        .with_color([0.2, 0.2, 0.2]);

    let ant = tile::Tile::new()
        .with_position(0.0, 0.0)
        .with_size(2.0 / 160.0, 2.0 / 100.0)
        .with_color([1.0, 0.0, 0.0]);

    // Time Control

    let steptime = 1000.;
    let mut elapse = 0.;

    support::start_loop(|delta| {
        use glium::Surface;
        use glium::glutin::Event;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        elapse += delta;
        if elapse > steptime {
            println!("Step");
            elapse = 0.;
        }

        target
            .draw(&grid.vertex_buffer(&display).unwrap(),
                  &grid.indices(),
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default())
            .unwrap();

        target
            .draw(&ant.vertex_buffer(&display).unwrap(),
                  &ant.indices(),
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
