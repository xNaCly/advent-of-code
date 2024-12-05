use std::ops::Index;

use crate::point::Point;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Grid {
    vec: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn parse(s: &str) -> Self {
        Grid::from_vec(s.lines().map(String::from).collect())
    }

    pub fn from_vec(v: Vec<String>) -> Self {
        let height = v.len();
        let vec: Vec<Vec<u8>> = v
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().map(|x| x as u8).collect())
            .collect();
        let width = vec.get(0).unwrap_or(&vec![]).len();
        Grid { vec, height, width }
    }

    pub fn columns(&self) -> Vec<Vec<u8>> {
        let row_length = self.vec[0].len();
        let mut columns = Vec::with_capacity(row_length);
        for i in 0..row_length {
            columns[i] = Vec::with_capacity(row_length);
            for byte in &self.vec[i] {
                columns[i].push(*byte);
            }
        }
        columns
    }

    pub fn rows(&self) -> Vec<Vec<u8>> {
        self.vec.clone()
    }
}

impl Index<Point> for Grid {
    type Output = u8;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.vec[index.x as usize][index.y as usize]
    }
}
