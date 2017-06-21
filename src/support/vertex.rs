
#[derive(Copy, Clone, Default)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn new(x: f32, y: f32, color: [f32; 3]) -> Vertex {
        Vertex {
            position: [x, y],
            color: color,
        }
    }

    pub fn from_points(x: f32, y: f32) -> Vertex {
        Vertex {
            position: [x, y],
            color: [1., 1., 1.],
        }
    }
}
