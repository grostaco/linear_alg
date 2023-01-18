use gloo::console::log;
use yew::prelude::*;

use crate::{
    components::{DimensionInput, Mat, Nav},
    math::Mat2d,
};

#[function_component(Home)]
pub fn home() -> Html {
    let dims = use_state(|| (3, 3));
    let dim_cb = {
        let dims = dims.clone();
        Callback::from(move |(m, n)| dims.set((m, n)))
    };

    let (m, n) = *dims;
    html! {
        <>
        <Nav />

        <main>
            <Mat::<f64, 13, 13> {m} {n}/>
            <DimensionInput {dim_cb} />
        </main>
        </>
    }
}
