use crate::types::{DistanceMatrix, Points, Tour};
use rand::{Rng, RngExt, seq::SliceRandom};

pub fn rand_tour(n: usize, matrix: &DistanceMatrix) -> Tour {
    let mut rng = rand::rng();
    let mut perm = vec![0usize; n];
    perm.iter_mut().enumerate().for_each(|(i, v)| *v = i);
    perm.shuffle(&mut rng);
    let distance = matrix.tour_distance(&perm);
    Tour {
        points: perm,
        distance,
    }
}
