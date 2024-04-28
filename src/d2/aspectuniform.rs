#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AspectUniform {
    pub aspect: f32,
    _padding: [f32;3]
}

impl AspectUniform {
    pub fn new() -> Self {
        Self {
            aspect: 0.0,
            _padding: [0.0;3]
        }
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}
