#![crate_name = "rs_ant"]
#![deny(missing_docs, missing_debug_implementations,
    missing_copy_implementations, trivial_casts, trivial_numeric_casts,
    unstable_features, unused_import_braces, unused_qualifications)]
#![allow(dead_code)]

//! Crate Documentation

#[macro_use]
extern crate glium;

mod support;
mod world;
mod tile;
mod grid;
mod world_draw;

use support::Drawable;

macro_rules! quick_draw {
    ($display: ident, $target: ident, $program: ident, [$($sel: ident),+]) => (
        $($target
            .draw(
                &$sel.vertex_buffer(&$display).unwrap(),
                &$sel.indices(),
                &$program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();)+
    )
}

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

    let gw = 160;
    let gh = 90;

    let grid = grid::Grid::new()
        .with_cols(gw)
        .with_rows(gh)
        .with_border(false)
        .with_color([0.0, 0.0, 0.0]);

    let mut world = world::World::new()
        .with_width(gw)
        .with_height(gh)
        .with_ant(gw / 2, gh / 2, 1);
    world.generate();

    // Time Control

    let mut steptime = 1.0;
    let mut elapse = 0.0;
    let mut running = false;
    let mut render = true;

    support::start_loop(|delta| {
        use glium::Surface;
        use glium::glutin::Event;

        if running {
            elapse += delta;
        }
        if elapse > steptime {
            world.step();
            elapse = 0.;
        }

        if render {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);

            quick_draw!(display, target, program, [world, grid]);
            // target
            //     .draw(
            //         &grid.vertex_buffer(&display).unwrap(),
            //         &grid.indices(),
            //         &program,
            //         &glium::uniforms::EmptyUniforms,
            //         &Default::default(),
            //     )
            //     .unwrap();

            target.finish().expect("Could not finish! ;)");
        }

        for ev in display.poll_events() {
            use glium::glutin::VirtualKeyCode as VK;

            match ev {
                Event::Closed => return support::Action::Stop,
                Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(key)) => {
                    match key {
                        VK::Equals => steptime /= 2.0,
                        VK::Minus => steptime *= 2.0,
                        VK::R => running = true,
                        VK::S => running = false,
                        VK::Q => render = false,
                        VK::W => render = true,
                        VK::Escape => return support::Action::Stop,
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        support::Action::Continue
    });
}
