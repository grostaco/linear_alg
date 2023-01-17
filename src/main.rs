use crate::math::{GaussElimIter, Mat2d};

mod components;
mod math;

fn main() {
    let mat = Mat2d::from([[0., 2., 1., 1.], [3., -7., -6., 1.], [0., -1., -1., 1.]]);
    let g = GaussElimIter::from(mat);
    println!("{:#?}", g.collect::<Vec<_>>());
}
