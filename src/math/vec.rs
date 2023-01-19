use std::{
    fmt::Debug,
    ops::{Div, Index, IndexMut, Mul, Sub},
    slice,
};

use num_traits::identities;

#[derive(Clone, PartialEq)]
pub struct Vec<T> {
    vec: std::vec::Vec<T>,
}

impl<T> Vec<T> {
    pub fn zeros(n: usize) -> Self
    where
        T: identities::Zero + Copy,
    {
        Self {
            vec: vec![T::zero(); n],
        }
    }

    pub fn ones(n: usize) -> Self
    where
        T: identities::One + Copy,
    {
        Self {
            vec: vec![T::one(); n],
        }
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.vec.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn get(&self, n: usize) -> Option<&T> {
        self.vec.get(n)
    }

    pub fn resize(&mut self, n: usize, value: T)
    where
        T: Clone,
    {
        self.vec.resize(n, value)
    }
}

impl<T> Sub<&Vec<T>> for &Vec<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec<T>;
    fn sub(self, rhs: &Vec<T>) -> Self::Output {
        assert!(
            self.vec.len() == rhs.vec.len(),
            "two vectors in subtraction must have the same length"
        );

        Vec {
            vec: self
                .vec
                .iter()
                .zip(rhs.iter())
                .map(|(a, b)| *a - *b)
                .collect(),
        }
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

impl<T, F> Mul<F> for &Vec<T>
where
    F: Copy,
    T: Mul<F, Output = T> + identities::Zero + Copy,
{
    type Output = Vec<T>;

    fn mul(self, rhs: F) -> Self::Output {
        Vec {
            vec: self.vec.iter().map(|x| *x * rhs).collect(),
        }
    }
}

impl<T, F> Div<F> for &Vec<T>
where
    F: Copy,
    T: Div<F, Output = T> + identities::Zero + Copy,
{
    type Output = Vec<T>;

    fn div(self, rhs: F) -> Self::Output {
        Vec {
            vec: self.vec.iter().map(|x| *x / rhs).collect(),
        }
    }
}

impl<T: Debug> Debug for Vec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<T, const N: usize> From<[T; N]> for Vec<T> {
    fn from(arr: [T; N]) -> Self {
        Vec { vec: arr.into() }
    }
}

impl<T> From<std::vec::Vec<T>> for Vec<T> {
    fn from(vec: std::vec::Vec<T>) -> Self {
        Vec { vec }
    }
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec = iter.into_iter().collect();
        Vec { vec }
    }
}
