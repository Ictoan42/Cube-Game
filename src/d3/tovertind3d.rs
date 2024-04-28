use super::vertex::Vertex3D;

pub trait ToVertInd3D {
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>);
}

impl<T, U> ToVertInd3D for (T, U)
where
    T: ToVertInd3D,
    U: ToVertInd3D
{
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        let (mut verts1, mut inds1) = self.0.to_vert_ind();
        let (mut verts2, mut inds2) = self.1.to_vert_ind();

        for ind in inds2.iter_mut() {
            *ind += verts1.len() as u16;
        }

        verts1.extend_from_slice(&mut verts2);
        inds1.extend_from_slice(&mut inds2);

        (verts1, inds1)
    }
}

impl<T> ToVertInd3D for Vec<T>
where
    T: ToVertInd3D,
{
    fn to_vert_ind(&self) -> (Vec<Vertex3D>, Vec<u16>) {
        let mut verts: Vec<Vertex3D> = vec![];
        let mut inds: Vec<u16> = vec![];

        for tvi in self.iter() {
            let (mut v, mut i) = tvi.to_vert_ind();

            for ind in i.iter_mut() {
                *ind += verts.len() as u16;
            }

            verts.extend_from_slice(&mut v);
            inds.extend_from_slice(&mut i);
        }

        (verts, inds)
    }
}

