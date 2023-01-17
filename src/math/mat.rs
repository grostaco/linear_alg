use super::vec;
use std::{
    fmt::Debug,
    ops::{Div, Index, IndexMut, Mul, Sub},
    slice,
};

use num_traits::identities;

#[derive(Clone, Copy)]
pub struct Mat2d<T, const M: usize, const N: usize> {
    mat: [vec::Vec<T, M>; N],
}

impl<T: Copy, const M: usize, const N: usize> Mat2d<T, M, N> {
    pub fn zeros() -> Self
    where
        T: identities::Zero,
    {
        Self {
            mat: [vec::Vec::zeros(); N],
        }
    }

    pub fn ones() -> Self
    where
        T: identities::One,
    {
        Self {
            mat: [vec::Vec::ones(); N],
        }
    }

    pub fn identity() -> Option<Self>
    where
        T: identities::One + identities::Zero,
    {
        if M != N {
            return None;
        }

        let mut mat = Self::zeros();

        for i in 0..N {
            mat.mat[i][i] = T::one();
        }

        Some(mat)
    }

    pub fn swap_row(&mut self, from: usize, to: usize) {
        self.mat.swap(from, to);
    }

    pub fn row_reduced(&self) -> Self
    where
        T: identities::Zero
            + PartialEq
            + Div<T, Output = T>
            + Mul<T, Output = T>
            + Sub<T, Output = T>
            + Copy,
    {
        let mut mat = self.clone();
        for i in 0..M.min(N) {
            // find a non-zero pivot
            if mat[i][i] == T::zero() {
                // There exists a non-zero pivot in the ith column
                if let Some(idx) = mat.iter().position(|x| T::zero().eq(&x[i])) {
                    mat.swap_row(i, idx);
                } else {
                    continue;
                }
            }
            for j in i + 1..N {
                if mat[j][i] != T::zero() {
                    let scale = mat[j][i] / mat[i][i];
                    mat[j] = mat[j].sub(&mat[i].mul(scale));
                }
            }
        }

        mat
    }

    pub fn shape(&self) -> (usize, usize) {
        (M, N)
    }

    pub fn iter(&self) -> slice::Iter<'_, vec::Vec<T, M>> {
        self.mat.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, vec::Vec<T, M>> {
        self.mat.iter_mut()
    }
}

impl<T: Debug, const M: usize, const N: usize> Debug for Mat2d<T, M, N> {
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

impl<T, const M: usize, const N: usize> Index<usize> for Mat2d<T, M, N> {
    type Output = vec::Vec<T, M>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Mat2d<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mat[index]
    }
}

impl<T: identities::Zero + Copy, const M: usize, const N: usize> From<[[T; M]; N]>
    for Mat2d<T, M, N>
{
    fn from(arr: [[T; M]; N]) -> Self {
        let mut mat = Self::zeros();
        for (vec, arr) in mat.iter_mut().zip(arr) {
            *vec = arr.into();
        }
        mat
    }
}
