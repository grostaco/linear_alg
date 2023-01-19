use math::{GaussElimIter, Mat2d};

pub mod components;
mod math;
mod routes;

fn main() {
    //let mat = Mat2d::from([[1., 5., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //let iter = GaussElimIter::from(mat);
    //println!("{:#?}", iter.collect::<Vec<_>>());
    yew::Renderer::<routes::Home>::new().render();
}
