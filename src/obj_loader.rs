#[derive(Clone,Copy,Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,

}
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces : Vec<[usize; 3]>,
}

