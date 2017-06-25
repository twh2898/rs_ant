
extern crate glium;

// MARK: Exports
pub mod shaders;
pub mod vertex;

pub use self::vertex::Vertex;

use std::time::{Instant, Duration};

// MARK: Traits

pub trait Drawable {
    type I;

    fn vertex_buffer<'a, D>(
        &self,
        display: &'a D,
    ) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
    where
        D: glium::backend::Facade;

    fn indices(&self) -> Self::I;
}

// MARK: Loop

pub enum Action {
    Continue,
    Stop,
}

fn as_sec(elapsed: Duration) -> f32 {
    elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1000000000.0
}

pub fn start_loop<F>(mut callback: F)
where
    F: FnMut(f32) -> Action,
{
    let start = Instant::now();
    let mut last = as_sec(start.elapsed());

    loop {
        let curr = as_sec(start.elapsed());
        let delta: f32 = curr - last;
        last = curr;

        match callback(delta) {
            Action::Stop => break,
            Action::Continue => (),
        }
    }
}
