
use cgmath::Vector2;

use super::{shape::Shape, tovertind2d::ToVertInd2D, vertex::Vertex2D};

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub pos: Vector2<f32>,
    pub rot: f32, // radians
    pub height: f32,
    pub width: f32,
    pub layer: u8,
    pub texindex: u32,
    pub flipped: bool,
    pub opacity: f32
}

impl Rectangle {
    pub fn new(
        width: f32,
        height: f32,
        pos: Vector2<f32>,
        rot: f32,
        layer: u8,
        texindex: u32,
        flipped: bool,
        opacity: f32
    ) -> Self {
        Self {width, height, pos, rot, layer, texindex, opacity, flipped}
    }
    pub fn set_opacity(&mut self, new_opacity: f32) {
        self.opacity = new_opacity
    }
}

impl Shape for Rectangle {
    fn get_pos(&self) -> Vector2<f32> {
        self.pos
    }
    fn set_pos(&mut self, p: Vector2<f32>) {
        self.pos = p
    }
    fn get_rot(&self) -> f32 {
        self.rot
    }
    fn set_rot(&mut self, r: f32) {
        self.rot = r
    }
}

impl ToVertInd2D for Rectangle {
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        let xoff = self.width / 2.0;
        let yoff = self.height / 2.0;
        let tex_index = self.texindex;
        let alpha = self.opacity;
        let depth = 0.5 - (self.layer as f32 / 32.0);
        let tx = if self.flipped {0.0} else {1.0};
        let mut verts = vec![
            Vertex2D {pos: [-xoff, -yoff ], tex_coords: [    tx, 1.0], depth, tex_index, alpha},
            Vertex2D {pos: [ xoff, -yoff ], tex_coords: [1.0-tx, 1.0], depth, tex_index, alpha},
            Vertex2D {pos: [-xoff,  yoff ], tex_coords: [    tx, 0.0], depth, tex_index, alpha},
            Vertex2D {pos: [ xoff,  yoff ], tex_coords: [1.0-tx, 0.0], depth, tex_index, alpha}
        ];

        // rotate all vertices
        for v in verts.iter_mut() {
            v.rotate_around_point_rad(self.rot, [0.0;2].into());
        }

        // translate all vertices
        for v in verts.iter_mut() {
            v.pos[0] = v.pos[0] + self.pos.x;
            v.pos[1] = v.pos[1] + self.pos.y;
        }

        let inds = vec![
            0, 1, 2,
            1, 3, 2
        ];

        (
            verts,
            inds
        )
    }
    fn layer(&self) -> u8 {
        self.layer
    }
}

impl ToVertInd2D for &Rectangle {
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        (*self).to_vert_ind()
    }
    fn layer(&self) -> u8 {
        (*self).layer()
    }
}

pub fn depth_sort(mut inp: Vec<impl ToVertInd2D>) -> Vec<impl ToVertInd2D> {
    inp.sort_unstable_by(|a, b| a.layer().partial_cmp(&b.layer()).unwrap());
    inp
}
