use crate::tsplib;

pub type Points = Vec<(f32, f32)>;

pub trait PointStorage<'a> {
    fn data_points(&'a self) -> &'a Points;
}

pub struct Tour {
    pub points: Vec<usize>,
    pub distance: i64,
}

impl Tour {
    pub fn new(n_points: usize) -> Self {
        Self {
            points: vec![0usize; n_points],
            distance: 0i64,
        }
    }

    pub fn with_distance(perm: &[usize], distance: i64) -> Self {
        Self {
            points: Vec::from(perm),
            distance,
        }
    }

    pub fn from_perm(perm: &[usize]) -> Self {
        Self {
            points: Vec::from(perm),
            distance: 0i64,
        }
    }

    pub fn swap(&mut self, i: usize, j: usize, delta: i64) {
        self.points.swap(i, j);
        self.distance += delta;
    }
}

pub struct DistanceMatrix {
    matrix: Vec<Vec<f32>>,
    n: usize,
}

impl DistanceMatrix {
    pub fn new(points: &Points) -> Self {
        let n = points.len();
        let mut matrix = vec![vec![0.0f32; n]; n];
        for i in 0..n - 1 {
            for j in i + 1..n {
                let (x0, y0) = &points[i];
                let (x1, y1) = &points[j];
                let dist = ((x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1)).sqrt();
                matrix[i][j] = dist;
                matrix[j][i] = dist;
            }
        }

        Self { matrix, n }
    }

    pub fn dist(&self, i: usize, j: usize) -> f32 {
        self.matrix[i][j]
    }

    pub fn swap_delta(&self, tour: &Tour, i: usize, j: usize) -> f32 {
        let n = tour.points.len();
        let i = tour.points[i];
        let j = tour.points[j];
        let prev_i = tour.points[(i as i64 - 1 + n as i64) as usize % n];
        let next_i = tour.points[(i + 1) % n];
        let prev_j = tour.points[(j as i64 - 1 + n as i64) as usize % n];
        let next_j = tour.points[(j + 1) % n];

        // eprintln!("{prev_i}, {i}, {next_i} :: {prev_j}, {j}, {next_j}");

        let old_cost = self.dist(prev_i, i)
            + self.dist(i, next_i)
            + self.dist(prev_j, j)
            + self.dist(j, next_j);
        let new_cost = self.dist(prev_i, j)
            + self.dist(prev_j, j)
            + self.dist(next_i, i)
            + self.dist(i, next_j);

        // eprintln!("old={old_cost}, new={new_cost}");
        new_cost - old_cost
    }

    pub fn tour_distance(&self, tour: &[usize]) -> i64 {
        let mut total = 0f64;
        for i in 0..self.n {
            let next = (i + 1) % self.n;
            total += self.dist(tour[i], tour[next]) as f64;
        }

        total.round() as i64
    }
}

pub struct TspSolution {
    tour: Tour,
}

impl TspSolution {
    pub fn new(tour: Tour) -> Self {
        Self { tour }
    }

    pub fn get_tour(&self) -> &Tour {
        &self.tour
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.00001f32
    }

    #[test]
    fn test_distance_matrix_swap_delta_correctness() {
        let dist = vec![
            (1.0f32, 1.0f32),
            (4.0f32, 1.0f32),
            (4.0f32, 4.0f32),
            (1.0f32, 4.0f32),
        ];
        let matrix = DistanceMatrix::new(&dist);
        let tour = Tour::from_perm(&[0, 1, 2, 3]);
        let tour2 = Tour::from_perm(&[0, 1, 3, 2]);

        assert!(close(matrix.dist(0, 1), 3.0f32));
        assert!(close(matrix.dist(2, 3), 3.0f32));
        assert!(close(matrix.dist(0, 2), 3f32 * std::f32::consts::SQRT_2));

        assert_eq!(matrix.tour_distance(&tour.points), 4 * 3i64);
        assert_eq!(matrix.tour_distance(&tour2.points), 2 * 3i64 + 8);

        let delta = matrix.swap_delta(&tour, 2, 3);
        const EXPECTED: f32 = 6f32 * (std::f32::consts::SQRT_2 - 1f32);
        assert!(
            close(delta, EXPECTED),
            "Swap delta={delta:.3} != expected delta={expected:.3}"
        );
    }
}
