use crate::types::{DistanceMatrix, Points, TspSolution};

pub trait TspProcedure {
    fn initialize(&self);
    fn run(&self, points: &Points, matrix: &DistanceMatrix) -> TspSolution;
}
