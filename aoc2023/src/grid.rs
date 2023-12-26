use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    grid: Vec<T>,
    m: usize,
    n: usize,
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn new(width: usize) -> Self {
        Grid {
            grid: vec![],
            m: 0,
            n: width,
        }
    }
    pub fn from_2d(grid2d: Vec<Vec<T>>) -> Self {
        let m = grid2d.len();
        let n = grid2d.first().unwrap().len();
        Grid {
            grid: grid2d.into_iter().flatten().collect(),
            m,
            n,
        }
    }
    pub fn push_row(&mut self, row: Vec<T>) {
        debug_assert!(self.n == row.len());
        self.grid.extend(row);
        self.m += 1;
    }
    pub fn duplicate(e: T, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        Grid {
            grid: vec![e; width * height],
            m: width,
            n: height,
        }
    }

    pub fn m(&self) -> usize {
        self.m
    }
    pub fn n(&self) -> usize {
        self.n
    }
}

impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.m {
            unsafe {
                write!(
                    f,
                    "{}",
                    std::str::from_utf8_unchecked(&self.grid[i * self.m..i * (self.m + 1)])
                )?
            }
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(index.0 < self.m && index.1 < self.n);
        unsafe { self.grid.get_unchecked(self.n * index.0 + index.1) }
    }
}
impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        debug_assert!(index[0] < self.m && index[1] < self.n);
        unsafe { self.grid.get_unchecked(self.n * index[0] + index[1]) }
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        debug_assert!(index.0 < self.m && index.1 < self.n);
        unsafe { self.grid.get_unchecked_mut(self.n * index.0 + index.1) }
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        debug_assert!(index[0] < self.m && index[1] < self.n);
        unsafe { self.grid.get_unchecked_mut(self.n * index[0] + index[1]) }
    }
}

impl<T> Grid<T> {
    pub fn iter<'a>(&'a self) -> GridRowIter<'a, T> {
        GridRowIter::<'a, T> {
            grid_ref: self,
            i: 0,
        }
    }
}

pub struct GridRowIter<'a, T> {
    grid_ref: &'a Grid<T>,
    i: usize,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.grid_ref.m {
            self.i += 1;

            Some(&self.grid_ref.grid[(self.i - 1) * self.grid_ref.n..self.i * self.grid_ref.n])
        } else {
            None
        }
    }
}
//impl<T> Iterator for Grid<T> {
//    type Item = GridRowIter<T>;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        todo!()
//    }
//}
//
//pub struct GridRowIter<T> {
//    i: i32,
//}
//
//impl<T> Iterator for GridRowIter<T> {
//
//}
