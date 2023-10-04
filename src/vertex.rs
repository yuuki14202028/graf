
#[derive(Copy, Clone)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
    pub(crate) color: [f32; 4]
}

implement_vertex!(Vertex, position, normal, color);