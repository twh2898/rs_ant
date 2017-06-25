
extern crate glium;

use support::{Vertex, Drawable};

pub struct Grid {
    cols: usize,
    rows: usize,
    border: bool,
    color: [f32; 3],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cols: 2,
            rows: 2,
            border: false,
            color: [0.2, 0.2, 0.2],
        }
    }

    pub fn with_cols(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn with_color(mut self, color: [f32; 3]) -> Self {
        self.color = color;
        self
    }
}

impl Drawable for Grid {
    type I = glium::index::NoIndices;

    fn vertex_buffer<'a, D>(
        &self,
        display: &'a D,
    ) -> Result<glium::VertexBuffer<Vertex>, glium::vertex::BufferCreationError>
    where
        D: glium::backend::Facade,
    {
        let mut shape: Vec<Vertex> = Vec::new();

        for col in 1..self.cols {
            let x = (col as f32 / self.cols as f32) * 2.0 - 1.0;

            let v1 = Vertex::from(x, -1., self.color.clone());
            let v2 = Vertex::from(x, 1., self.color.clone());

            shape.push(v1);
            shape.push(v2);
        }

        for row in 1..self.rows {
            let y = (row as f32 / self.rows as f32) * 2.0 - 1.0;

            let v1 = Vertex::from(-1., y, self.color.clone());
            let v2 = Vertex::from(1., y, self.color.clone());

            shape.push(v1);
            shape.push(v2);
        }

        if self.border {
            shape.push(Vertex::from(-1., -1., self.color.clone()));
            shape.push(Vertex::from(1., -1., self.color.clone()));

            shape.push(Vertex::from(1., -1., self.color.clone()));
            shape.push(Vertex::from(1., 1., self.color.clone()));

            shape.push(Vertex::from(1., 1., self.color.clone()));
            shape.push(Vertex::from(-1., 1., self.color.clone()));

            shape.push(Vertex::from(-1., 1., self.color.clone()));
            shape.push(Vertex::from(-1., -1., self.color.clone()));
        }

        glium::VertexBuffer::new(display, &shape)
    }

    fn indices(&self) -> glium::index::NoIndices {
        glium::index::NoIndices(glium::index::PrimitiveType::LinesList)
    }
}
