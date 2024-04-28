use std::f32::consts::PI;

use cgmath::Vector2;

pub trait Shape {
    fn get_pos(&self) -> Vector2<f32>;
    fn set_pos(&mut self, p: Vector2<f32>);
    fn get_rot(&self) -> f32;
    fn set_rot(&mut self, r: f32);
    fn rotate_rad(&mut self, r: f32) {
        let cr = self.get_rot();
        self.set_rot( (cr + r ) % (2.0 * PI) );
    }
    fn rotate_deg(&mut self, r: f32) {
        let r_rad = ( r / 360.0 ) * 2.0 * PI;
        self.rotate_rad(r_rad);
    }
    fn translate(&mut self, t: Vector2<f32>) {
        let p = self.get_pos();
        self.set_pos(p + t);
    }
    fn rotate_around_point_rad(&mut self, r: f32, p: Vector2<f32>) {
        self.rotate_rad(r);

        self.set_pos({
            let cp = self.get_pos();

            // thank you Mark Booth
            // https://stackoverflow.com/questions/620745/c-rotating-a-vector-around-a-certain-point
            [
                ((cp.x - p.x) * r.cos()) - ((cp.y - p.y) * r.sin()) + p.x,
                ((cp.x - p.x) * r.sin()) + ((cp.y - p.y) * r.cos()) + p.y
            ].into()
        });
    }
    fn rotate_around_point_deg(&mut self, r: f32, p: Vector2<f32>) {
        let r_rad = ( r / 360.0 ) * 2.0 * PI;
        self.rotate_around_point_rad(r_rad, p);
    }
}


