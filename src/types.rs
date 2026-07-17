use crate::tsplib;

pub type Points = Vec<(f32, f32)>;

pub trait PointStorage<'a> {
    fn data_points(&'a self) -> &'a Points;
}
