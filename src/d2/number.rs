use cgmath::Vector2;

use crate::config::NUMBER_TEX_INDEX_START;

use super::{rectangle::Rectangle, shape::Shape, tovertind2d::ToVertInd2D, vertex::Vertex2D};

#[derive(Clone)]
pub struct Number {
    value: u32,
    pos: Vector2<f32>,
    rot: f32,
    layer: u8,
    opacity: f32,
    digit_w: f32,
    digit_h: f32,
    digit_gap: f32,
    right_aligned: bool,
}

impl Number {
    pub fn new(
        value: u32,
        pos: Vector2<f32>,
        rot: f32,
        layer: u8,
        opacity: f32,
        digit_w: f32,
        digit_h: f32,
        digit_gap: f32,
        right_aligned: bool,
    ) -> Self {
        Self {value, pos, rot, layer, opacity, digit_w, digit_h, digit_gap, right_aligned}
    }
    pub fn set(&mut self, v: u32) {
        self.value = v;
    }
}

impl Shape for Number {
    fn get_pos(&self) -> Vector2<f32> {
        self.pos
    }
    fn get_rot(&self) -> f32 {
        self.rot
    }
    fn set_pos(&mut self, p: Vector2<f32>) {
        self.pos = p;
    }
    fn set_rot(&mut self, r: f32) {
        self.rot = r;
    }
}

impl ToVertInd2D for Number {
    fn layer(&self) -> u8 {
        self.layer
    }
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        let v_str = format!("{}", self.value);

        let mut rects: Vec<Rectangle> = vec![];

        for (i,d) in v_str.chars().enumerate() {
            let texindex = d.to_digit(10).unwrap() + NUMBER_TEX_INDEX_START;
            let pos: Vector2<f32> = match self.right_aligned {
                false => {
				    [(self.digit_w + self.digit_gap) * i as f32,0.0].into()
                }
                true => {
                    let total_digits = v_str.len();
                    let offset_per_digit = ( self.digit_w + self.digit_gap ) * -1.0;
                    let x = offset_per_digit * (total_digits - i) as f32;
                    [x,0.0].into()
                }
            };
            let mut r = Rectangle::new(
                self.digit_w,
				self.digit_h,
				pos,
				0.0,
				self.layer,
				texindex,
				true,
				self.opacity
			);
			r.translate(self.pos);
			r.rotate_around_point_rad(self.rot, self.pos);
			rects.push(r);
        }

        rects.to_vert_ind()
    }
}

