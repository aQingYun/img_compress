use std::fs;

pub fn get_file_size(path: &String) -> f64 {
  fs::metadata(path).map(|m| m.len() as f64).unwrap_or(0.0)
}