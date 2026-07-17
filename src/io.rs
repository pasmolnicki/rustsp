use std::{
    error::Error,
    path::{Path, PathBuf},
};

/// Returns absolute path to `problemset` folder in this project
pub fn get_problemset_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(std::env::current_dir()?.join("problemset"))
}

/// Returns all tsp filenames in the `problemset` directory
pub fn problemset_filenames() -> Result<Vec<String>, Box<dyn Error>> {
    let dirs = std::fs::read_dir(get_problemset_path()?)?;
    let mut result = Vec::new();
    for dir in dirs {
        let entry = dir?;
        if entry.file_type()?.is_file() && entry.path().extension().unwrap() == "tsp" {
            result.push(entry.file_name().into_string().unwrap());
        }
    }
    Ok(result)
}

pub fn load_problemset_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string(
        get_problemset_path()?.join(file_name),
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn test_get_problemset_path_doesnt_fail() {
        assert!(get_problemset_path().is_ok());
    }

    #[test]
    fn test_problemset_filenames_finds_tsp_files() {
        let filenames = problemset_filenames();
        assert!(
            filenames.is_ok(),
            "Filenames should be discovered without fail: {}",
            filenames.err().as_ref().unwrap().deref().to_string()
        );
        let filenames = filenames.unwrap();
        assert!(
            filenames.len() > 0,
            ".tsp files should be present in the problemset directory"
        );
    }

    #[test]
    fn test_problemset_load_file_loads_existing_tsp_file() {
        let filenames = problemset_filenames().unwrap();
        let first = filenames.first();
        assert!(
            first.is_some(),
            ".tsp files should be present in the problemset directory"
        );
        let file = load_problemset_file(&first.unwrap());
        assert!(
            file.is_ok(),
            "File: {} should be properly loaded: {}",
            first.unwrap(),
            file.err().unwrap().as_ref().deref().to_string()
        );
    }
}
