use crate::tsplib;

pub enum FileFormat {
    Tsplib(tsplib::InputData),
}
