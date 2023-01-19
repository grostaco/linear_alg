use yew::prelude::*;

use crate::{
    components::{DimensionInput, MatEdit, Nav, Steps},
    math::Mat2d,
};

#[function_component(Home)]
pub fn home() -> Html {
    let dims = use_state(|| (3, 3));
    let mat = use_state(Mat2d::<f64, 64, 64>::zeros);
    let dim_cb = {
        let dims = dims.clone();
        Callback::from(move |(m, n)| dims.set((m, n)))
    };

    let mat_cb = {
        let mat = mat.clone();
        Callback::from(move |new_mat| {
            mat.set(new_mat);
        })
    };

    let (m, n) = *dims;
    html! {
        <>
        <Nav />

        <main>
            <MatEdit::<f64, 64, 64> mat={*mat} {m} {n} onchange={mat_cb}/>
            <div class="dflex dflex-row dflex-gap-sm">
                <DimensionInput {dim_cb} />
            </div>

            <Steps::<f64, 64, 64> mat={*mat} {m} {n}/>

        </main>
        </>
    }
}
