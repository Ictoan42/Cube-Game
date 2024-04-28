use cgmath::{Deg, Quaternion, Rotation3, Vector3};

use super::{tovertind3d::ToVertInd3D, vertex::Vertex3D};

pub struct Column {
    pos: Vector3<f32>,
    rot: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    colour: [[f32;4];6]
}

impl Column {
    pub fn new(pos: Vector3<f32>, rotopt: Option<Quaternion<f32>>, width: f32, height: f32, initcol: [f32;4]) -> Self {
        let rot = rotopt.or(
            Some(Quaternion::from_angle_y(Deg(0.0)))
        ).unwrap();
        Self {
            pos, rot, width, height, colour: [initcol;6]
        }
    }
    pub fn set_side_col(&mut self, col: [f32;4], ind: u8) {
        if ind >= 6 { return }

        self.colour[ind as usize] = col;
    }
}

impl ToVertInd3D for Column {
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        let mut verts = get_column_vertices(self.height, self.width, [0.0;4]).to_vec();

        verts.iter_mut().for_each(|v| {
            v.translate(self.pos);
            v.rotate_around(self.pos, self.rot)
        });

        verts.chunks_mut(4).enumerate().for_each(|(i, va)| {
            let col = self.colour[i];
            va.iter_mut().for_each(|v| {
                v.col = col;
            });
        });

        
        (
            verts,
            COLUMN_INDICES.to_vec()
        )
    }
}

impl ToVertInd3D for &Column {
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        (*self).to_vert_ind()
    }
}

fn get_column_vertices(height: f32, width: f32, initcol: [f32;4]) -> [Vertex3D;24] {
    let w: f32 = width / 2.0;
    [
    // bottom
    Vertex3D {pos: [-w,   0.0, w], col: initcol, norm: [ 0.0, 1.0, 0.0]},
    Vertex3D {pos: [ w,   0.0, w], col: initcol, norm: [ 0.0, 1.0, 0.0]},
    Vertex3D {pos: [ w,   0.0,-w], col: initcol, norm: [ 0.0, 1.0, 0.0]},
    Vertex3D {pos: [-w,   0.0,-w], col: initcol, norm: [ 0.0, 1.0, 0.0]},
    // front
    Vertex3D {pos: [-w,height,-w], col: initcol, norm: [ 0.0, 0.0,-1.0]},
    Vertex3D {pos: [ w,height,-w], col: initcol, norm: [ 0.0, 0.0,-1.0]},
    Vertex3D {pos: [ w,   0.0,-w], col: initcol, norm: [ 0.0, 0.0,-1.0]},
    Vertex3D {pos: [-w,   0.0,-w], col: initcol, norm: [ 0.0, 0.0,-1.0]},
    // right
    Vertex3D {pos: [ w,height, w], col: initcol, norm: [ 1.0, 0.0, 0.0]},
    Vertex3D {pos: [ w,height,-w], col: initcol, norm: [ 1.0, 0.0, 0.0]},
    Vertex3D {pos: [ w,   0.0,-w], col: initcol, norm: [ 1.0, 0.0, 0.0]},
    Vertex3D {pos: [ w,   0.0, w], col: initcol, norm: [ 1.0, 0.0, 0.0]},
    // back
    Vertex3D {pos: [-w,   0.0, w], col: initcol, norm: [ 0.0, 0.0, 1.0]},
    Vertex3D {pos: [ w,   0.0, w], col: initcol, norm: [ 0.0, 0.0, 1.0]},
    Vertex3D {pos: [ w,height, w], col: initcol, norm: [ 0.0, 0.0, 1.0]},
    Vertex3D {pos: [-w,height, w], col: initcol, norm: [ 0.0, 0.0, 1.0]},
    // top
    Vertex3D {pos: [-w,height,-w], col: initcol, norm: [ 0.0,-1.0, 0.0]},
    Vertex3D {pos: [ w,height,-w], col: initcol, norm: [ 0.0,-1.0, 0.0]},
    Vertex3D {pos: [-w,height, w], col: initcol, norm: [ 0.0,-1.0, 0.0]},
    Vertex3D {pos: [ w,height, w], col: initcol, norm: [ 0.0,-1.0, 0.0]},
    // left
    Vertex3D {pos: [-w,   0.0,-w], col: initcol, norm: [-1.0, 0.0, 0.0]},
    Vertex3D {pos: [-w,   0.0, w], col: initcol, norm: [-1.0, 0.0, 0.0]},
    Vertex3D {pos: [-w,height,-w], col: initcol, norm: [-1.0, 0.0, 0.0]},
    Vertex3D {pos: [-w,height, w], col: initcol, norm: [-1.0, 0.0, 0.0]},
    ]
}

const COLUMN_INDICES: [u16;36] = [
    // bottom
    3, 0, 2,
    2, 0, 1,
    // front
    4, 7, 5,
    5, 7, 6,
    // right
    8, 9, 11,
    11, 9, 10,
    // back
    15, 13, 12,
    14, 13, 15,
    // top
    18, 16, 17,
    18, 17, 19,
    // left
    21, 20, 23,
    23, 20, 22
];
