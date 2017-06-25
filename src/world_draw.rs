
extern crate glium;

use support::{self, Vertex};
use world::World;

impl support::Drawable for World {
    type I = glium::index::NoIndices;

    fn vertex_buffer<'a, D>(
        &self,
        display: &'a D,
    ) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
    where
        D: glium::backend::Facade,
    {
        let mut shape: Vec<Vertex> = Vec::new();
        let ant = self.ant();

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let element = self.state(row, col).unwrap();
                let color = if row == ant.0 && col == ant.1 {
                    [1.0, 0.0, 0.0]
                } else {
                    match element {
                        true => [1.0, 1.0, 1.0],
                        false => [0.2, 0.2, 0.2],
                    }
                };
                let w: f32 = 1.0 / self.cols() as f32;
                let h: f32 = 1.0 / self.rows() as f32;
                let x: f32 = row as f32 / self.cols() as f32 * 2.0 - 1.0;
                let y: f32 = col as f32 / self.rows() as f32 * 2.0 - 1.0;

                shape.push(Vertex::from(x, y, color));
                shape.push(Vertex::from(x + w, y, color));
                shape.push(Vertex::from(x, y + h, color));
                shape.push(Vertex::from(x + w, y + h, color));
            }
        }

        glium::VertexBuffer::new(display, &shape)
    }

    fn indices(&self) -> glium::index::NoIndices {
        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip)
    }
}
