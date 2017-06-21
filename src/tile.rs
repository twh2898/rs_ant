
extern crate glium;

use support::Vertex;

pub struct Tile {
    x: (f32, f32),
    y: (f32, f32),
}

impl Tile {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Tile {
        Tile {
            x: (x, x + width),
            y: (y, y + height),
        }
    }

    pub fn vertex_buffer<'a, D>
        (&self,
         display: &'a D)
         -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
        where D: glium::backend::Facade
    {
        let color = [1., 1., 1.];
        let v1 = Vertex::new(self.x.0, self.y.0, color.clone());
        let v2 = Vertex::new(self.x.1, self.y.0, color.clone());
        let v3 = Vertex::new(self.x.0, self.y.1, color.clone());
        let v4 = Vertex::new(self.x.1, self.y.1, color);
        let shape = vec![v1, v2, v3, v3, v2, v4];

        glium::VertexBuffer::new(display, &shape)
    }
}
