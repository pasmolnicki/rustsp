use crate::tsp_procedure::TspProcedure;
use crate::types::{DistanceMatrix, Points, Tour, TspSolution};
use crate::utils::rand_tour;

pub struct LocalSearch;

impl TspProcedure for LocalSearch {
    fn initialize(&self) {}
    fn run(&self, points: Points, matrix: &DistanceMatrix) -> TspSolution {
        let n = points.len();
        let mut tour = rand_tour(n, matrix);
        let mut best_swap = None;
        let mut best_delta;

        loop {
            best_delta = f32::MAX;
            for i in 0..n - 1 {
                for j in i + 1..n {
                    let swap_delta = matrix.swap_delta(&tour, i, j);
                    if swap_delta < best_delta {
                        best_delta = swap_delta;
                        best_swap = Some((i, j));
                    }
                }
            }

            if let Some((i, j)) = best_swap {
                tour.swap(i, j, best_delta as i64);
            } else {
                break;
            }
        }

        TspSolution::new(tour)
    }
}
