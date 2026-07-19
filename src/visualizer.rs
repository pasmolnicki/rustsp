use std::{error::Error, path::PathBuf};

use plotters::prelude::*;

use crate::types::{Points, Tour};

// Treats points as a cycle, so each point is connected to the next one
// and the last one is connected to the first one
pub fn visualize(
    tour: &Tour,
    points: &Points,
    file_name: String,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut tour_points = vec![(0f64, 0f64); tour.points.len()];
    tour_points.iter_mut().enumerate().for_each(|(i, (x, y))| {
        let (px, py) = &points[tour.points[i]];
        *x = *px as f64;
        *y = *py as f64;
    });

    if tour_points.is_empty() {
        return Err("cannot visualize an empty point set".into());
    }

    let output_path = {
        let path = PathBuf::from(file_name);
        if path.extension().is_none() {
            path.with_extension("png")
        } else {
            path
        }
    };

    let (min_x, max_x) = tour_points.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(min_x, max_x), (x, _)| (min_x.min(*x), max_x.max(*x)),
    );
    let (min_y, max_y) = tour_points.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(min_y, max_y), (_, y)| (min_y.min(*y), max_y.max(*y)),
    );

    let x_span = (max_x - min_x).abs();
    let y_span = (max_y - min_y).abs();
    let x_padding = if x_span == 0.0 { 1.0 } else { x_span * 0.1 };
    let y_padding = if y_span == 0.0 { 1.0 } else { y_span * 0.1 };

    let result_path = output_path.clone();
    let root = BitMapBackend::new(&output_path, (1000, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root).margin(20).build_cartesian_2d(
        (min_x - x_padding)..(max_x + x_padding),
        (min_y - y_padding)..(max_y + y_padding),
    )?;

    let mut cycle_points = tour_points.clone();
    if cycle_points.len() > 1 {
        cycle_points.push(cycle_points[0]);
    }

    chart.draw_series(LineSeries::new(cycle_points, &BLUE.mix(0.8)))?;

    chart.draw_series(
        tour_points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 5, RED.filled())),
    )?;

    root.present()?;
    Ok(result_path)
}
