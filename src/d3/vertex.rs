use cgmath::{Quaternion, Rotation, Vector3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3D {
    pub pos: [f32;3],
    pub norm: [f32;3],
    pub col: [f32;4]
}

impl Vertex3D {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x4
        ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS
        }
    }
    pub fn translate(&mut self, trn: Vector3<f32>) {
        self.pos = ( Vector3::from(self.pos) + trn ).into();
    }
    pub fn rotate_around(&mut self, point: Vector3<f32>, rot: Quaternion<f32>) {
        let vec: Vector3<f32> = Vector3::from(self.pos) - point;
        let newvec = rot.rotate_vector(vec);
        self.pos = (point + newvec).into();
        self.norm = rot.rotate_vector(self.norm.into()).into();
    }
}


