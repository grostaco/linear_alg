use super::vec;
use std::{
    fmt::Debug,
    ops::{Div, Index, IndexMut, Mul, Sub},
    slice,
};

use num_traits::{identities, Float};

#[derive(Clone, PartialEq)]
pub struct Mat2d<T> {
    mat: Vec<vec::Vec<T>>,
}

#[derive(Debug)]
pub enum Step<T> {
    Swap { from: usize, to: usize },
    Sub { scale: T, from: usize, to: usize },
}

impl<T: Copy> Mat2d<T> {
    pub fn zeros(m: usize, n: usize) -> Self
    where
        T: identities::Zero,
    {
        Self {
            mat: vec![vec::Vec::zeros(n); m],
        }
    }

    pub fn ones(m: usize, n: usize) -> Self
    where
        T: identities::One,
    {
        Self {
            mat: vec![vec::Vec::ones(n); m],
        }
    }

    pub fn identity(m: usize, n: usize) -> Option<Self>
    where
        T: identities::One + identities::Zero,
    {
        if m != n {
            return None;
        }

        let mut mat = Self::zeros(m, n);

        for i in 0..n {
            mat.mat[i][i] = T::one();
        }

        Some(mat)
    }

    pub fn swap_row(&mut self, from: usize, to: usize) {
        self.mat.swap(from, to);
    }

    pub fn row_reduced(&self) -> Self
    where
        T: Float,
    {
        let mut mat = self.clone();
        let (m, n) = self.shape();

        for i in 0..m.min(n) {
            // find a non-zero pivot
            if mat[i][i] == T::zero() {
                // There exists a non-zero pivot in the ith column
                if let Some(idx) = mat.iter().skip(i).position(|x| !T::zero().eq(&x[i])) {
                    mat.swap_row(i, idx);
                } else {
                    continue;
                }
            }
            for j in i + 1..m {
                if mat[j][i] != T::zero() {
                    let scale = mat[j][i] / mat[i][i];
                    mat[j] = mat[j].sub(&mat[i].mul(scale));
                }
            }
        }

        mat
    }

    pub fn row_reduced_verbose(&self) -> Vec<(Self, Step<T>)>
    where
        T: Float,
    {
        let mut steps = Vec::new();
        let mut mat = self.clone();
        let (m, n) = self.shape();

        for i in 0..m.min(n) {
            // find a non-zero pivot
            if mat[i][i] == T::zero() {
                // There exists a non-zero pivot in the ith column
                if let Some(idx) = mat.iter().skip(i).position(|x| !T::zero().eq(&x[i])) {
                    mat.swap_row(i, idx);
                    steps.push((mat.clone(), Step::Swap { from: i, to: idx }));
                } else {
                    continue;
                }
            }
            for j in i + 1..m {
                if mat[j][i] != T::zero() {
                    let scale = mat[j][i] / mat[i][i];
                    mat[j] = mat[j].sub(&mat[i].mul(scale));
                    steps.push((
                        mat.clone(),
                        Step::Sub {
                            scale,
                            from: i,
                            to: j,
                        },
                    ));
                }
            }
        }

        steps
    }

    pub fn rref(&self) -> Self
    where
        T: Float,
    {
        let mut mat = self.row_reduced();
        let (m, n) = self.shape();

        for i in (0..n).rev() {
            if let Some(idx) = mat
                .iter()
                .map(|x| x[i])
                .rev()
                .position(|x| !x.eq(&T::zero()))
            {
                let idx = m - idx - 1;
                if mat[idx][i] != T::zero() {
                    mat[idx] = mat[idx].div(mat[idx][i])
                }

                for j in (0..idx).rev() {
                    if mat[j][i] != T::zero() {
                        let scale = mat[j][i] / mat[idx][i];
                        mat[j] = mat[j].sub(&mat[idx].mul(scale));
                    }
                }
            }
        }

        mat
    }

    pub fn rank(&self) -> usize
    where
        T: Float,
    {
        let rref = self.rref();
        rref.iter()
            .map(|v| v.iter().any(|x| !x.is_zero()) as usize)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn shape(&self) -> (usize, usize) {
        let m = self.mat.len();
        let n = self.mat.get(0).map(|x| x.len()).unwrap_or(0);
        (m, n)
    }

    pub fn iter(&self) -> slice::Iter<'_, vec::Vec<T>> {
        self.mat.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, vec::Vec<T>> {
        self.mat.iter_mut()
    }

    pub fn transpose(&self) -> Self {
        todo!()
    }

    pub fn resize(&mut self, m: usize, n: usize)
    where
        T: identities::Zero,
    {
        for v in &mut self.mat {
            v.resize(n, T::zero())
        }
        self.mat.resize(m, vec::Vec::zeros(n));
    }
}

impl<T: Debug> Debug for Mat2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[\n\t{}\n]",
            self.mat
                .iter()
                .fold(String::new(), |acc, a| if acc.is_empty() {
                    format!("{a:?}")
                } else {
                    format!("{acc},\n\t{a:?}")
                })
        )
    }
}

impl<T> Index<usize> for Mat2d<T> {
    type Output = vec::Vec<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl<T> IndexMut<usize> for Mat2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mat[index]
    }
}

impl<T: identities::Zero + Copy, const M: usize, const N: usize> From<[[T; N]; M]> for Mat2d<T> {
    fn from(arr: [[T; N]; M]) -> Self {
        let mut mat = Self::zeros(M, N);
        for (vec, arr) in mat.iter_mut().zip(arr) {
            *vec = arr.into();
        }
        mat
    }
}
