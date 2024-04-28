use std::iter::zip;

use cgmath::Vector2;

use crate::d3::columngrid::ColumnGrid;

use super::{rectangle::Rectangle, shape::Shape, tovertind2d::ToVertInd2D};

// represents the top-down wireframe 2D view of a columngrid
#[derive(Clone, Debug)]
pub struct Net {
    sidelen: u8,
    squares: Vec< Vec< NetSquare > >,
    layer: u8,
    pos: Vector2<f32>,
    rot: f32, // DEGREES
    opacity: f32
}

impl Net {
    pub fn from_columngrid(
        cg: &ColumnGrid,
        texindex: u32,
        layer: u8,
        pos: Vector2<f32>,
        scale: f32,
        edgethickness: f32,
        opacity: f32
    ) -> Self {
        let sidelen = cg.sidelen;
        let mut squares = vec![
            vec![
                NetSquare::blank(
                    texindex,
                    layer,
                    [0.0;2].into(), // value never used
                    0.0, // neither this one
                    scale,
                    edgethickness,
                    false,
                    opacity
                );sidelen.into()
            ];sidelen.into()
        ];

        let offset = scale * f32::from(sidelen - 1) * 0.5;

        for x in 0i16..sidelen.into() {
            for y in 0i16..sidelen.into() {
                let sq = &mut squares[x as usize][y as usize];
                let current = sample_column_grid(x, y, cg);

                if current != 0 {
                    sq.fill = true;
                }

                sq.top    = sample_column_grid(x, y-1, cg) != current;
                sq.bottom = sample_column_grid(x, y+1, cg) != current;
                sq.right  = sample_column_grid(x-1, y, cg) != current;
                sq.left   = sample_column_grid(x+1, y, cg) != current;

                let mut sqpos: Vector2<f32> = [0.0;2].into();
                sqpos.x = sqpos.x + x as f32 * scale;
                sqpos.y = sqpos.y - y as f32 * scale;

                sq.pos = sqpos;

                // offset by net pos
                sq.pos += pos;

                sq.pos.x -= offset;
                sq.pos.y += offset;
            }
        }

        Self {
            sidelen,
            squares,
            layer,
            pos,
            rot: 0.0,
            opacity
        }
    }
    pub fn set_opacity(&mut self, o: f32) {
        self.opacity = o;
        for s in self.squares.iter_mut().flatten() {
            s.opacity = o;
        }
    }
    pub fn square_debug_info(&self) -> String {
        let mut out: String = "".into();

        for y in 0..self.sidelen {
            for x in 0..self.sidelen {
                out += &self.squares[x as usize][y as usize].side_debug_info();
            }
            out += "\n";
        }

        out
    }
    pub fn is_identical(&self, other: &Net) -> bool {
        let iter = zip(self.squares.iter().flatten(), other.squares.iter().flatten());
        for (s1, s2) in iter {
            if !{
                s1.top == s2.top &&
                s1.left == s2.left &&
                s1.right == s2.right &&
                s1.bottom == s2.bottom
            } {
                return false
            }
        }
        true
    }
}

impl ToVertInd2D for Net {
    fn layer(&self) -> u8 {
        self.layer
    }
    fn to_vert_ind(&self) -> (Vec<super::vertex::Vertex2D>, Vec<u16>) {
        self.squares.to_vert_ind()
    }
}

impl ToVertInd2D for &Net {
    fn layer(&self) -> u8 {
        (*self).layer()
    }
    fn to_vert_ind(&self) -> (Vec<super::vertex::Vertex2D>, Vec<u16>) {
        (*self).to_vert_ind()
    }
}

impl Shape for Net {
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

// represents a single square on a net image
#[derive(Clone, Debug, PartialEq)]
pub struct NetSquare {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    texindex: u32,
    layer: u8,
    pos: Vector2<f32>,
    rot: f32,
    scale: f32,
    thickness: f32,
    pub fill: bool,
    opacity: f32
}

impl NetSquare {
    pub fn blank(
        texindex: u32,
        layer: u8,
        pos: Vector2<f32>,
        rot: f32,
        scale: f32,
        thickness: f32,
        fill: bool,
        opacity: f32
    ) -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            texindex,
            layer,
            pos,
            rot,
            scale,
            thickness,
            fill,
            opacity
        }
    }
    pub fn side_debug_info(&self) -> String {
        let mut out: String = "".into();

        if self.top {out += "T"} else {out += "t"}
        if self.bottom {out += "B"} else {out += "b"}
        if self.left {out += "L"} else {out += "l"}
        if self.right {out += "R"} else {out += "r"}
        out += " ";

        out
    }
}

impl ToVertInd2D for NetSquare {
    fn layer(&self) -> u8 {
        self.layer
    }
    fn to_vert_ind(&self) -> (Vec<super::vertex::Vertex2D>, Vec<u16>) {
        let mut rects: Vec<Rectangle> = vec![];

        let base = Rectangle::new(
            self.thickness,
            self.scale + self.thickness * 0.9,
            [0.0;2].into(), // to be reset on each clone
            0.0, // same here
            self.layer,
            self.texindex,
            false,
            self.opacity
        );

        let selfrepr = Rectangle::new(
            self.scale,
            self.scale,
            self.pos,
            self.rot,
            self.layer - 1,
            self.texindex,
            false,
            if self.fill {0.2 * self.opacity} else {0.0}
        );
        rects.push(selfrepr);

        if self.top {
            let mut top = base.clone();
            top.translate(self.pos);
            top.translate([0.0,self.scale/2.0].into());
            top.rotate_deg(90.0);
            top.rotate_around_point_rad(self.rot, self.pos);
            rects.push(top);
        }
        if self.bottom {
            let mut bottom = base.clone();
            bottom.translate(self.pos);
            bottom.translate([0.0,-self.scale/2.0].into());
            bottom.rotate_deg(90.0);
            bottom.rotate_around_point_rad(self.rot, self.pos);
            rects.push(bottom);
        }
        if self.left {
            let mut left = base.clone();
            left.translate(self.pos);
            left.translate([self.scale/2.0,0.0].into());
            left.rotate_around_point_rad(self.rot, self.pos);
            rects.push(left);
        }
        if self.right {
            let mut right = base.clone();
            right.translate(self.pos);
            right.translate([-self.scale/2.0,0.0].into());
            right.rotate_around_point_rad(self.rot, self.pos);
            rects.push(right);
        }

        rects.to_vert_ind()
    }
}

impl ToVertInd2D for &NetSquare {
    fn layer(&self) -> u8 {
        (*self).layer
    }
    fn to_vert_ind(&self) -> (Vec<super::vertex::Vertex2D>, Vec<u16>) {
        (*self).to_vert_ind()
    }
}

impl Shape for NetSquare {
    fn set_rot(&mut self, r: f32) {
        self.rot = r
    }
    fn get_rot(&self) -> f32 {
        self.rot
    }
    fn set_pos(&mut self, p: Vector2<f32>) {
        self.pos = p
    }
    fn get_pos(&self) -> Vector2<f32> {
        self.pos
    }
}

fn sample_column_grid(x: i16, y: i16, cg: &ColumnGrid) -> u8 {
    let sidelen: i16 = cg.sidelen.into();
    if x < 0 || y < 0 {
        0
    } else if x >= sidelen || y >= sidelen {
        0
    } else {
        cg.columns[x as usize][y as usize]
    }
}
