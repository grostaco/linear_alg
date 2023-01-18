use yew::prelude::*;

use crate::{
    components::{Mat, Nav},
    math::Mat2d,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <Nav />

        <Mat::<f64, 13, 13> mat={Mat2d::zeros()} m=4 n=3/>
        </>
    }
}
