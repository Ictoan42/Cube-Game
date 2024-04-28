use cgmath::{InnerSpace, Vector2};
use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::rectangle::Rectangle;

pub fn convert_mouse_coords(pos: PhysicalPosition<f64>, size: PhysicalSize<u32>, aspect: f32) -> Vector2<f32> {
    let mut x = pos.x as f32 / size.width as f32;
    x = (x * 2.0) - 1.0;
    let mut y = pos.y as f32 / size.height as f32;
    y = (y * 2.0) - 1.0;
    x *= aspect;

    [x,-y].into()
}

/// Expects inputs in GUI space, not pixel space
pub fn is_in_circle(mpos: Vector2<f32>, centre: Vector2<f32>, radius: f32) -> bool {
    let dist = (centre - mpos).magnitude();
    dist < radius
}

/// Expects inputs in GUI space, not pixel space
pub fn is_in_rounded_rect(pos: Vector2<f32>, rect: &Rectangle, radius: f32) -> bool {

    // instead of testing against rotated coordinates, just rotate
    // the mouse position to match
    let mpos: Vector2<f32> = {[
        (pos.x * rect.rot.cos()) - (pos.y * rect.rot.sin()),
        (pos.x * rect.rot.sin()) + (pos.y * rect.rot.cos())
    ].into()};

    let vert_rect = Rectangle::new(
        rect.width - 2.0 * radius,
        rect.height,
        rect.pos,
        rect.rot,
        0,
        255,
        false,
        0.0
    );
    let hor_rect = Rectangle::new(
        rect.width,
        rect.height - 2.0 * radius,
        rect.pos,
        rect.rot,
        0,
        0,
        false,
        0.0
    );

    let tl: Vector2<f32> = [  (rect.pos.x - rect.width/2.0)+radius,   (rect.pos.y + rect.height/2.0)-radius  ].into();
    let tr: Vector2<f32> = [  (rect.pos.x + rect.width/2.0)-radius,   (rect.pos.y + rect.height/2.0)-radius  ].into();
    let bl: Vector2<f32> = [  (rect.pos.x - rect.width/2.0)+radius,   (rect.pos.y - rect.height/2.0)+radius  ].into();
    let br: Vector2<f32> = [  (rect.pos.x + rect.width/2.0)-radius,   (rect.pos.y - rect.height/2.0)+radius  ].into();

    is_in_rect(mpos, &vert_rect) ||
    is_in_rect(mpos, &hor_rect) ||
    is_in_circle(mpos, tl, radius) ||
    is_in_circle(mpos, tr, radius) ||
    is_in_circle(mpos, bl, radius) ||
    is_in_circle(mpos, br, radius)
}

/// Expects inputs in GUI space, not pixel space
pub fn is_in_rect(pos: Vector2<f32>, rect: &Rectangle) -> bool {
    
    // instead of testing against rotated coordinates, just rotate
    // the mouse position to match
    let mpos: Vector2<f32> = {[
        (pos.x * rect.rot.cos()) - (pos.y * rect.rot.sin()),
        (pos.x * rect.rot.sin()) + (pos.y * rect.rot.cos())
    ].into()};

    let x1 = rect.pos.x + rect.width/2.0;
    let x2 = rect.pos.x - rect.width/2.0;
    let y1 = rect.pos.y + rect.height/2.0;
    let y2 = rect.pos.y - rect.height/2.0;

    let is_in_x = x1 > mpos.x && mpos.x > x2;
    let is_in_y = y1 > mpos.y && mpos.y > y2;

    is_in_y && is_in_x
}
