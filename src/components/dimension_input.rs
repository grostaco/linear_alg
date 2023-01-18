use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, use_state, Callback, Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(3)]
    pub m: usize,
    #[prop_or(3)]
    pub n: usize,

    pub dim_cb: Callback<(usize, usize), ()>,
}

#[function_component(DimensionInput)]
pub fn dimension_input(props: &Props) -> Html {
    let m = use_state(|| props.m);
    let n = use_state(|| props.n);

    let m_ref = use_node_ref();
    let n_ref = use_node_ref();

    {
        let m = m.clone();
        let n = n.clone();
        let m_ref = m_ref.clone();
        let n_ref = n_ref.clone();
        use_effect_with_deps(
            move |m_ref| {
                let element: HtmlInputElement = m_ref.cast().expect("m input element");
                element.set_value(&m.to_string());
            },
            m_ref,
        );

        use_effect_with_deps(
            move |n_ref| {
                let element: HtmlInputElement = n_ref.cast().expect("n input element");
                element.set_value(&n.to_string());
            },
            n_ref,
        );
    }

    let m_onchange = {
        let m = m.clone();
        let n = n.clone();
        let dim_cb = props.dim_cb.clone();

        Callback::from(move |e: web_sys::Event| {
            let v = e
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse()
                .expect("valid number");
            m.set(v);
            dim_cb.emit((v, *n))
        })
    };

    let n_onchange = {
        let dim_cb = props.dim_cb.clone();

        Callback::from(move |e: web_sys::Event| {
            let v = e
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse()
                .expect("valid number");
            n.set(v);
            dim_cb.emit((*m, v))
        })
    };

    html! {
        <div style="margin-top: 1em;" class="dflex dflex-gap-tn">
            <input size=1 onchange={m_onchange} class="dim-input" ref={m_ref}/>
            <span>{"x"}</span>
            <input size=1 onchange={n_onchange} class="dim-input" ref={n_ref}/>
        </div>
    }
}
