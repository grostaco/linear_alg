use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Mul, Sub},
    slice,
};

use num_traits::identities;

#[derive(Copy, Clone)]
pub struct Vec<T, const N: usize> {
    vec: [T; N],
}

impl<T: Copy, const N: usize> Vec<T, N> {
    pub fn zeros() -> Self
    where
        T: identities::Zero,
    {
        Self {
            vec: [T::zero(); N],
        }
    }

    pub fn ones() -> Self
    where
        T: identities::One,
    {
        Self { vec: [T::one(); N] }
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.vec.iter_mut()
    }
}

impl<T, const N: usize> Sub<&Vec<T, N>> for &Vec<T, N>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Vec<T, N>;
    fn sub(self, rhs: &Vec<T, N>) -> Self::Output {
        let mut vec = self.vec.clone();
        for (vec, rhs) in vec.iter_mut().zip(rhs.iter()) {
            *vec = *vec - *rhs;
        }
        Vec { vec }
    }
}

// impl<T, const N: usize> Mul<&Vec<T, N>> for &Vec<T, N>
// where
//     T: Mul<Output = T> + identities::Zero + Copy,
// {
//     type Output = Vec<T, N>;

//     fn mul(self, rhs: &Vec<T, N>) -> Self::Output {
//         let mut vec = self.vec.clone();
//         for (vec, rhs) in self.vec.iter().zip(vec.iter_mut()) {
//             *vec = *vec * *rhs;
//         }
//         Vec { vec }
//     }
// }

// impl<T, F, const N: usize> Mul<F> for &mut Vec<T, N>
// where
//     F: Copy,
//     T: Mul<F, Output = T> + identities::Zero + Copy,
// {
//     type Output = Self;

//     fn mul(self, rhs: F) -> Self::Output {
//         for v in &mut self.vec {
//             *v = *v * rhs;
//         }
//         self
//     }
// }

impl<T, F, const N: usize> Mul<F> for &Vec<T, N>
where
    F: Copy,
    T: Mul<F, Output = T> + identities::Zero + Copy,
{
    type Output = Vec<T, N>;

    fn mul(self, rhs: F) -> Self::Output {
        let mut vec = self.vec.clone();
        for v in &mut vec {
            *v = *v * rhs;
        }
        Vec { vec }
    }
}

impl<T: Debug, const N: usize> Debug for Vec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}

impl<T: Copy, const N: usize> Index<usize> for Vec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T: Copy, const N: usize> IndexMut<usize> for Vec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<T: Copy, const N: usize> From<[T; N]> for Vec<T, N> {
    fn from(arr: [T; N]) -> Self {
        Vec { vec: arr }
    }
}
