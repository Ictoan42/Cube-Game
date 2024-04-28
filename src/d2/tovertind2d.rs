use super::vertex::Vertex2D;

pub trait ToVertInd2D {
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>);
    fn layer(&self) -> u8;
}

impl<T> ToVertInd2D for Vec<T>
where
    T: ToVertInd2D,
{
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        let mut verts: Vec<Vertex2D> = vec![];
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
    fn layer(&self) -> u8 {
        unimplemented!()
    }
}

impl<T> ToVertInd2D for Box<T>
where
    T: ToVertInd2D + ?Sized
{
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        (**self).to_vert_ind()
    }
    fn layer(&self) -> u8 {
        (**self).layer()
    }
}

impl<T> ToVertInd2D for &Box<T>
where
    T: ToVertInd2D + ?Sized
{
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        (**self).to_vert_ind()
    }
    fn layer(&self) -> u8 {
        (**self).layer()
    }
}

impl<T> ToVertInd2D for &Vec<T>
where
    T: ToVertInd2D
{
    fn layer(&self) -> u8 {
        (*self).layer()
    }
    fn to_vert_ind(&self) -> (Vec<Vertex2D>, Vec<u16>) {
        (*self).to_vert_ind()
    }
}
