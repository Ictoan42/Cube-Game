use cgmath::Vector2;

use super::shape::Shape;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    pub pos: [f32;2],
    pub depth: f32,
    pub tex_coords: [f32;2],
    pub tex_index: u32,
    pub alpha: f32
}

impl Vertex2D {
    const ATTRIBS: [wgpu::VertexAttribute; 5] =
        wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32,
            2 => Float32x2,
            3 => Uint32,
            4 => Float32
        ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS
        }
    }
    pub fn translate(&mut self, trn: Vector2<f32>) {
        self.pos = ( Vector2::from(self.pos) + trn ).into();
    }
}

impl Shape for Vertex2D {
    fn get_pos(&self) -> Vector2<f32> {
        self.pos.into()
    }
    fn set_pos(&mut self, p: Vector2<f32>) {
        self.pos = p.into()
    }
    fn get_rot(&self) -> f32 {
        0.0
    }
    fn set_rot(&mut self, _: f32) {}
}
