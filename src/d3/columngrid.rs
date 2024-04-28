use cgmath::{Deg, Quaternion, Rotation, Rotation3, Vector3};
use rand::{rngs::ThreadRng, Rng};

use super::{column::Column, tovertind3d::ToVertInd3D, vertex::Vertex3D};

#[derive(Clone, Debug)]
pub struct ColumnGrid {
    pub pos: Vector3<f32>,
    rot: Quaternion<f32>,
    pub sidelen: u8,
    pub columns: Vec<
        Vec<
            u8 // represents the height in that column
        >
    >
}

impl ColumnGrid {
    pub fn new(pos: Vector3<f32>, rotopt: Option<Quaternion<f32>>, sidelen: u8) -> Self {
        let rot = rotopt.or(
            Some(Quaternion::from_angle_y(Deg(0.0)))
        ).unwrap();

        Self {
            pos, rot, columns: vec![ vec![0;sidelen as usize];sidelen as usize ], sidelen
        }
    }
    pub fn set_column(&mut self, x: u8, y: u8, height: u8) {
        self.columns[x as usize][y as usize] = height;
    }
    pub fn translate(&mut self, trn: Vector3<f32>) {
        self.pos = self.pos + trn;
    }
    pub fn new_random(pos: Vector3<f32>, rotopt: Option<Quaternion<f32>>, sidelength: u8) -> Self {

        fn gen_random_column(x: u8, y: u8, r: &mut ThreadRng, size: u8) -> u8 {
            // carry out the weighted random generation
            // of one column's height
            //
            // this func is only a func to limit nesting
            
            let offsetmul: f64 = 1.0;

            let offset = ( (x as f64 + y as f64) / size as f64 ) * offsetmul;
            
            let height: f64 = r.gen::<f64>() * 2.0;

            let heightadj = f64::max(height - offset, 0.0) / 1.5;

            ( (size as f64) * heightadj ) as u8

        }

        let mut out = Self::new(pos, rotopt, sidelength);

        let mut rng = rand::thread_rng();

        for x in 0..sidelength {
            for y in 0..sidelength {
                out.set_column(x, y, gen_random_column(x, y, &mut rng, sidelength))
            }
        }

        out
    }
    pub fn random_change(&mut self) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..self.sidelen) as usize;
        let y = rng.gen_range(0..self.sidelen) as usize;
        let h = rng.gen_range(0..self.sidelen);
        self.columns[0][0] = 0;

        self.columns[x][y] = h;
    }
    pub fn random_changes(&mut self, ch: u32) {
        for _ in 0..ch {
            self.random_change()
        }
    }
    pub fn count_occupied_columns(&self) -> u32 {
        let mut count = 0;
        for c in self.columns.iter().flatten() {
            if *c > 0 {
                count += 1;
            }
        }
        count
    }
    pub fn highest_column(&self) -> u8 {
        let mut highest = 0;
        for c in self.columns.iter().flatten() {
            if *c > highest {
                highest = *c
            }
        }
        highest
    }
}

impl ToVertInd3D for ColumnGrid {
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>) {

        let mut columns: Vec<Column> = vec![];

        for x in 0..self.sidelen {
            for y in 0..self.sidelen {

                if self.columns[x as usize][y as usize] == 0 {
                    continue;
                }

                let mut pos: Vector3<f32> = self.pos;
                pos.x += x as f32;
                pos.z += y as f32;

                pos = self.rot.rotate_vector(self.pos - pos) + self.pos;

                let mut c = Column::new(
                    pos,
					Some(self.rot),
					1.005,
					self.columns[x as usize][y as usize] as f32,
					[1.0,0.0,0.0,1.0]
				);

				let top_bot_col_offset = (self.columns[x as usize][y as usize] as f32) / 25.0;

				let top_bot_col = [
                    1.0 - top_bot_col_offset,
                    0.1 + top_bot_col_offset,
                    0.1 + top_bot_col_offset,
                    1.0
				];

				// let front_back_col_offset = ( y as f32 ) / 200.0;
				let front_back_col_offset = 0.0;

				let front_back_col = [
                    0.91 - front_back_col_offset,
                    0.04 + front_back_col_offset,
                    0.04 + front_back_col_offset,
                    1.0
				];

				// let left_right_col_offset = ( x as f32 ) / 200.0;
				let left_right_col_offset = 0.0;

				let left_right_col = [
                    0.87 - left_right_col_offset,
                    0.08 + left_right_col_offset,
                    0.08 + left_right_col_offset,
                    1.0
				];

                // top and bottom
				c.set_side_col(top_bot_col, 0);
				c.set_side_col(top_bot_col, 4);
                // front and back
				c.set_side_col(front_back_col, 1);
				c.set_side_col(front_back_col, 3);
                // left and right
				c.set_side_col(left_right_col, 2);
				c.set_side_col(left_right_col, 5);

				columns.push(c);
            }
        }

        columns.to_vert_ind()
    }
}

impl ToVertInd3D for &ColumnGrid {
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        (*self).to_vert_ind()
    }
}
