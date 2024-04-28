use cgmath::Vector2;

use super::{shape::Shape, tovertind2d::ToVertInd2D, vertex::Vertex2D};

#[derive(Clone)]
pub struct Spiral {
    pos: Vector2<f32>,
    rot: f32,
    depth: f32,
    outer_radius: f32,
    inner_radius: f32,
    tex_index: u32,
    alpha: f32
}

impl Spiral {
    pub fn new(
        pos: Vector2<f32>,
        rot: f32,
        depth: f32,
        inner_radius: f32,
        outer_radius: f32,
        tex_index: u32,
        alpha: f32
    ) -> Self {
        Self { pos, rot, depth, outer_radius, inner_radius, tex_index, alpha }
    }
    pub fn set_depth(&mut self, d: f32) {
        self.depth = d;
    }
    pub fn set_opacity(&mut self, o: f32) {
        self.alpha = o;
    }
}

impl ToVertInd2D for Spiral {
    fn layer(&self) -> u8 {
        0
    }
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        let mut verts: Vec<Vertex2D> = vec![];
        let mut inds: Vec<u16> = vec![];

        let depth = self.depth;
        let tex_coords = [0.0,0.0];
        let tex_index = self.tex_index;
        let alpha = self.alpha;

        let mut first_out_point = Vertex2D {
            pos: [0.0,self.outer_radius], depth, tex_coords, tex_index, alpha
        };
        first_out_point.rotate_deg(0.0);

        let mut first_in_point = Vertex2D {
            pos: [0.0,self.inner_radius], depth, tex_coords, tex_index, alpha
        };
        first_in_point.rotate_deg(0.0);

        verts.push(first_out_point);
        verts.push(first_in_point);

        for ri32 in 1..=360 {
            let r = ri32 as f32;
            let depth = self.depth + r / ( 360.0 * 4.0 );
            gen_verts_deg(
                self.inner_radius,
                self.outer_radius,
                r,
                depth,
                &mut verts,
                &mut inds
            );
        }

        for v in verts.iter_mut() {
            v.translate(self.pos);
            v.alpha = self.alpha;
            v.tex_index = self.tex_index;
            v.tex_coords = v.pos;
        }

        (verts, inds)
    }
}

impl Shape for Spiral {
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

fn gen_verts_deg(in_r: f32, out_r: f32, rot: f32, depth: f32, vb: &mut Vec<Vertex2D>, ib: &mut Vec<u16>) {
    // most of these are overriden in the to_vert_ind impl
    let alpha = 0.0;
    let tex_coords = [0.0,0.0];
    let tex_index = 0;

    let mut in_point = Vertex2D {
        pos: [0.0,in_r], depth, tex_coords, tex_index, alpha
    };
    in_point.rotate_around_point_deg(rot, [0.0;2].into());

    let mut out_point = Vertex2D {
        pos: [0.0,out_r], depth, tex_coords, tex_index, alpha
    };
    out_point.rotate_around_point_deg(rot, [0.0;2].into());

    let last_out_ind = vb.len() as u16 - 2;
    let last_in_ind = vb.len() as u16 - 1;
    let this_out_ind = vb.len() as u16;
    let this_in_ind = vb.len() as u16 + 1;

    vb.push(out_point);
    vb.push(in_point);

    ib.extend_from_slice(&[last_out_ind, this_out_ind, last_in_ind]);
    ib.extend_from_slice(&[this_out_ind, this_in_ind, last_in_ind]);
    
}
