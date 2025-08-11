#[derive(Debug,Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        Matrix(slice.iter().map(|a| a.to_vec()).collect())
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}",self.0.iter().map(|arr|format!("({})",arr.iter().map(|ele|ele.to_string()).collect::<Vec<String>>().join(" "))).collect::<Vec<String>>().join("\n"))
    }
}