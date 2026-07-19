use std::{
    error::Error,
    io::{self, ErrorKind},
    path::{self, PathBuf},
};

use rustsp::{
    io::{load_problemset_file, load_problemset_file_with_base},
    local_search::LocalSearch,
    tsp_procedure::TspProcedure,
    tsplib::{self, ParseHeadersError},
    types::{DistanceMatrix, PointStorage},
    utils,
    visualizer::visualize,
};

/// Returns absolute path to project root directory - the 'rustsp' folder
fn get_base_path(args: &Vec<String>) -> Result<PathBuf, Box<dyn Error>> {
    let abs_path = path::absolute(&args[0])?;
    let base_path = abs_path.parent().unwrap();
    if let Some(f) = base_path.file_stem() {
        let name = f
            .to_str()
            .expect("Conversion into string of executable's parent directory failed");

        match name {
            "release" | "baseline" => {
                Ok(base_path.parent().unwrap().parent().unwrap().to_path_buf())
            }
            _ => Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Unknown parent directory name, expected 'release' or 'baseline'",
            ))),
        }
    } else {
        Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "Couldn't read the directory name of the binary file",
        )))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <tsp file>", args[0]);
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid number of arguments",
        )));
    }

    let base_path = get_base_path(&args)?;
    println!("{}", base_path.to_str().unwrap());
    let data = tsplib::InputData::from_str(&load_problemset_file_with_base(&base_path, &args[1])?)?;
    let n = data.dimension;
    let matrix = DistanceMatrix::new(data.data_points());
    // let tour = utils::rand_tour(n, &matrix);
    // let file_path = visualize(&tour, data.data_points(), "rand".to_owned())?;
    // let rand_len = matrix.tour_distance(&tour.points);

    let algo = LocalSearch::default();
    let solution = algo.run(data.data_points(), &matrix);
    println!("{}: {}", args[1], solution.get_tour().distance);

    /*let solution_filepath = visualize(
        solution.get_tour(),
        data.data_points(),
        "local_search".to_owned(),
    )?;*/

    /*println!(
        "Rand-len={rand_len}, LocalSearch={}",
        solution.get_tour().distance
    );*/

    /*println!(
        "Created files: {}, {}",
        file_path.to_str().unwrap(),
        solution_filepath.to_str().unwrap()
    );*/
    Ok(())
}
