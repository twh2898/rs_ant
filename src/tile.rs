
extern crate glium;

use support::Vertex;
use support::Drawable;

pub struct Tile {
    size: (f32, f32),
    x: (f32, f32),
    y: (f32, f32),
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            size: (1., 1.),
            x: (0., 1.),
            y: (0., 1.),
        }
    }

    pub fn from(x: f32, y: f32, width: f32, height: f32) -> Self {
        Tile {
            size: (width, height),
            x: (x, x + width),
            y: (y, y + height),
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = (x, x + self.size.0);
        self.y = (y, y + self.size.1);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = (width, height);
        self.x.1 = self.x.0 + width;
        self.y.1 = self.y.0 + height;
        self
    }
}

impl Drawable for Tile {
    type I = glium::index::NoIndices;

    fn vertex_buffer<'a, D>
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
        let shape = vec![v1, v2, v3, v4];

        glium::VertexBuffer::new(display, &shape)
    }

    fn indices(&self) -> glium::index::NoIndices {
        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip)
    }
}
