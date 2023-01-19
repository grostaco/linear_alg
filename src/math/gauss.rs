use std::ops::{Div, Mul, Sub};

use num_traits::identities;

use super::Mat2d;

// pub enum State {
//     Unsolvable,
//     Infinite,
//     Solvable,
// }

#[derive(Debug)]
pub enum Step<T> {
    Swap { from: usize, to: usize },
    Sub { scale: T, from: usize, to: usize },
}

pub struct GaussElimIter<T> {
    mat: Mat2d<T>,
    col: usize,
    row: usize,
}

impl<T> From<Mat2d<T>> for GaussElimIter<T> {
    fn from(mat: Mat2d<T>) -> Self {
        Self {
            mat,
            col: 0,
            row: 1,
        }
    }
}

impl<T> Iterator for GaussElimIter<T>
where
    T: identities::Zero + PartialEq + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    type Item = (Step<T>, Mat2d<T>);
    fn next(&mut self) -> Option<Self::Item> {
        let (m, n) = self.mat.shape();

        loop {
            if self.col >= m || self.row >= n {
                return None;
            }

            if self.mat[self.col][self.col] == T::zero() {
                // There exists a non-zero pivot in the ith column
                if let Some(idx) = self.mat.iter().position(|x| !T::zero().eq(&x[self.col])) {
                    self.mat.swap_row(self.col, idx);
                    return Some((
                        Step::Swap {
                            from: self.col,
                            to: idx,
                        },
                        self.mat.clone(),
                    ));
                } else {
                    self.col += 1;
                }
            }

            loop {
                if self.mat[self.col][self.row] != T::zero() {
                    let scale = self.mat[self.row][self.col] / self.mat[self.col][self.col];
                    self.mat[self.row] = self.mat[self.row].sub(&self.mat[self.col].mul(scale));
                    return Some((
                        Step::Sub {
                            scale,
                            from: self.col,
                            to: self.row,
                        },
                        self.mat.clone(),
                    ));
                }

                self.row += 1;
                if self.row == n {
                    self.col += 1;
                    self.row = self.col + 1;
                    break;
                }
            }
        }
    }
}
