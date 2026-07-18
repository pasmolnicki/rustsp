use crate::types::{self, PointStorage};
use std::{error::Error, num::ParseIntError, string::ParseError};

#[derive(Debug)]
pub struct ParseCoordError {
    message: String,
}

impl std::fmt::Display for ParseCoordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseCoordError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl ParseCoordError {
    pub fn new(reason: String) -> Self {
        Self {
            message: format!("Invalid NODE_COORD_SECTION: {reason}"),
        }
    }
}

#[derive(Debug)]
pub struct ParseHeadersError {
    message: String,
}

impl std::fmt::Display for ParseHeadersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseHeadersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl ParseHeadersError {
    fn new(header: &str) -> Self {
        Self {
            message: format!("Unexpected header: {header}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Tsp,
}

#[derive(Debug, PartialEq)]
pub enum EdgeWeightType {
    Euc2d,
}

#[derive(Debug)]
pub struct InputData {
    name: String,
    comment: String,
    data_type: DataType,
    dimension: usize,
    edge_weight_type: EdgeWeightType,
    points: types::Points,
}

impl<'a> PointStorage<'a> for InputData {
    fn data_points(&'a self) -> &'a types::Points {
        &self.points
    }
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            name: String::new(),
            comment: String::new(),
            data_type: DataType::Tsp,
            dimension: 0,
            edge_weight_type: EdgeWeightType::Euc2d,
            points: Vec::new(),
        }
    }
}

impl InputData {
    pub fn from_str(file: &str) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for line in file.split('\n') {
            if line.is_empty() || line == "NODE_COORD_SECTION" {
                continue;
            }

            // Try to split it by :
            let splitted = line.split(':').collect::<Vec<&str>>();
            if splitted.len() == 2 {
                let left = splitted[0].trim();
                let right = splitted[1].trim();

                match left {
                    "NAME" => {
                        ret.name = right.to_owned();
                    }
                    "COMMENT" => {
                        ret.comment.push_str(right);
                    }
                    "TYPE" => {
                        ret.data_type = DataType::Tsp;
                    }
                    "DIMENSION" => {
                        ret.dimension = right.parse()?;
                        ret.points.resize(ret.dimension, (0f32, 0f32));
                    }
                    "EDGE_WEIGHT_TYPE" => {
                        ret.edge_weight_type = EdgeWeightType::Euc2d;
                    }
                    &_ => {
                        return Err(Box::new(ParseHeadersError::new(left)));
                    }
                }
            } else {
                let space_split = line.split(' ').collect::<Vec<&str>>();
                if space_split.len() != 3 {
                    return Err(Box::new(ParseCoordError::new(format!(
                        "Invalid space split (len={}), at: {line}",
                        space_split.len()
                    ))));
                }
                let idx = space_split[0]
                    .parse::<usize>()?
                    .checked_sub(1)
                    .ok_or(Box::new(ParseCoordError::new(format!(
                        "Negative index at: {line}"
                    ))))?;
                let x = space_split[1].parse::<f32>()?;
                let y = space_split[2].parse::<f32>()?;
                ret.points[idx] = (x, y);
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::io::{load_problemset_file, problemset_filenames};

    #[test]
    fn test_tsp_file_is_properly_parsed() {
        const FILE_NAME: &str = "dj38.tsp";
        const NAME: &str = "dj38";
        const TYPE: DataType = DataType::Tsp;
        const DIMENSION: usize = 38usize;
        const EDGE_WEIGHT_TYPE: EdgeWeightType = EdgeWeightType::Euc2d;

        let file_name = problemset_filenames().unwrap();
        let file_name = file_name.iter().find(|name| *name == FILE_NAME);
        assert!(
            file_name.is_some(),
            "Couldn't find {FILE_NAME} in the problemset"
        );
        let file_str = load_problemset_file(&file_name.unwrap())
            .expect("Should load already existing .tsp file");

        let data = InputData::from_str(&file_str);
        assert!(
            data.is_ok(),
            "TspData wasn't parsed properly due to error: {}",
            data.err().unwrap().as_ref().deref().to_string()
        );

        let data = data.ok().unwrap();
        assert_eq!(data.name, NAME);
        assert_eq!(data.data_type, TYPE);
        assert_eq!(data.dimension, DIMENSION);
        assert_eq!(data.edge_weight_type, EDGE_WEIGHT_TYPE);
    }
}
