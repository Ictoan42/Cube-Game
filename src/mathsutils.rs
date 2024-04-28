use cgmath::{Vector1, Vector4, VectorSpace};

pub fn lerp(st: f32, en: f32, n: f32) -> f32 {
    let stv = Vector1::new(st);
    let env = Vector1::new(en);
    stv.lerp(env, n).x
}

pub fn lerp4d(st: [f32;4], en: [f32;4], n: f32) -> [f32;4] {
    let stv: Vector4<f32> = st.into();
    let env: Vector4<f32> = en.into();
    stv.lerp(env, n).into()
}
