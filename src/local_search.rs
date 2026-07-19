use crate::tsp_procedure::TspProcedure;
use crate::types::{DistanceMatrix, Points, Tour, TspSolution};
use crate::utils::rand_tour;

pub struct LocalSearch {
    n_iterations: fn(usize) -> usize,
}

impl Default for LocalSearch {
    fn default() -> Self {
        Self {
            n_iterations: |n| n / 2,
        }
    }
}

impl LocalSearch {
    pub fn new(n_iterations: fn(usize) -> usize) -> Self {
        Self { n_iterations }
    }

    fn step(&self, points: &Points, matrix: &DistanceMatrix) -> TspSolution {
        let n = points.len();
        let mut tour = rand_tour(n, matrix);
        let mut best_swap;
        let mut best_delta;

        loop {
            best_delta = f32::MAX;
            best_swap = None;

            for i in 0..n - 1 {
                for j in i + 1..n {
                    let swap_delta = matrix.swap_delta(&tour, i, j);
                    if swap_delta > best_delta {
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

impl TspProcedure for LocalSearch {
    fn initialize(&self) {}
    fn run(&self, points: &Points, matrix: &DistanceMatrix) -> TspSolution {
        let mut best_solution = None;
        let mut best_distance = i64::MAX;
        let n = (self.n_iterations)(points.len());

        for i in 0..n {
            let solution = self.step(points, matrix);
            let distance = solution.get_tour().distance;
            if (best_distance > distance) {
                best_distance = distance;
                best_solution = Some(solution);
            }
        }

        best_solution.unwrap()
    }
}
