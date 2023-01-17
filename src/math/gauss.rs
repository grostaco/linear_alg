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

pub struct GaussElimIter<T, const M: usize, const N: usize> {
    mat: Mat2d<T, M, N>,
    col: usize,
    row: usize,
}

impl<T, const M: usize, const N: usize> From<Mat2d<T, M, N>> for GaussElimIter<T, M, N> {
    fn from(mat: Mat2d<T, M, N>) -> Self {
        Self {
            mat,
            col: 0,
            row: 1,
        }
    }
}

impl<T, const M: usize, const N: usize> Iterator for GaussElimIter<T, M, N>
where
    T: identities::Zero
        + PartialEq
        + Div<T, Output = T>
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + Copy,
{
    type Item = (Step<T>, Mat2d<T, M, N>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.col == M.min(N) || self.row >= M.min(N) {
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
                if self.mat[self.row][self.col] != T::zero() {
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
                if self.row == N {
                    self.col += 1;
                    self.row = self.col + 1;
                    break;
                }
            }
        }
    }
}
