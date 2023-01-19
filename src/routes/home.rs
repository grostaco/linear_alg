use yew::prelude::*;

use crate::{
    components::{DimensionInput, MatEdit, Nav, Steps},
    math::Mat2d,
};

#[function_component(Home)]
pub fn home() -> Html {
    let dims = use_state(|| (3, 3));
    let mat = use_state(|| Mat2d::<f64>::zeros(3, 3));
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

    {
        let mat = mat.clone();
        use_effect_with_deps(
            move |dims| {
                let (m, n) = **dims;
                let mut rmat = (*mat).clone();
                rmat.resize(m, n);
                mat.set(rmat);
            },
            dims,
        )
    }

    html! {
        <>
        <Nav />

        <main>
            <h1>{"Input Matrix"}</h1>
            <MatEdit::<f64> mat={(*mat).clone()} onchange={mat_cb}/>
            <div class="dflex dflex-row dflex-gap-sm">
                <DimensionInput {dim_cb} />
            </div>

            <Steps::<f64> mat={(*mat).clone()}/>

        </main>
        </>
    }
}
