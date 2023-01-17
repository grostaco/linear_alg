use crate::math::Mat2d;

mod components;
mod math;

fn main() {
    let mat = Mat2d::from([[1., 2., 1., 1.], [3., -7., -6., 1.], [0., -1., -1., 1.]]);
    println!("{:#?}", mat.row_reduced());
}
