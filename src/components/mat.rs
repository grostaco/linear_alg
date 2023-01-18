use gloo::console::log;
use num_traits::identities;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlElement, MouseEvent, ShadowRootInit, ShadowRootMode};
use yew::{
    create_portal, function_component, html, use_effect_with_deps, use_node_ref, use_state,
    virtual_dom::VNode, Callback, Html, Properties, TargetCast, UseStateHandle,
};
use yew_hooks::use_effect_once;

use crate::math::Mat2d;

#[derive(Properties, PartialEq)]
pub struct Props<T, const M: usize, const N: usize>
where
    T: PartialEq + identities::Zero + Copy,
{
    #[prop_or(Mat2d::zeros())]
    pub mat: Mat2d<T, M, N>,
}
#[function_component(Mat)]
pub fn mat<T, const M: usize, const N: usize>(props: &Props<T, M, N>) -> Html
where
    T: PartialEq + identities::Zero + Copy,
{
    // let div_ref = use_node_ref();

    // {
    //     let div_ref = div_ref.clone();
    //     use_effect_with_deps(
    //         |div_ref| {
    //             let div = div_ref.cast::<HtmlElement>().expect("Unattached");
    //             let inner = div.inner_html();
    //             div.set_outer_html(&format!("<math>{inner}</math>"));
    //         },
    //         div_ref,
    //     )
    // }

    // let onclick = Callback::from(|e: MouseEvent| {
    //     let element: HtmlElement = e.target_unchecked_into();
    //     log!("Clicked");
    // });

    html! {
        <div>
        <MathElem />
        </div>
    }
}

#[function_component(MathElem)]
pub fn math_elem() -> Html {
    let host_ref = use_node_ref();
    let inner_host: UseStateHandle<Option<Element>> = use_state(Option::default);

    //let onclick = Callback::from(|_| log!("Clicked"));
    // let onclick_ref = use_node_ref();

    // {
    //     let onclick_ref = onclick_ref.clone();
    //     use_effect_with_deps(
    //         |onclick_ref| {
    //             let closure =
    //                 Closure::<dyn FnMut(MouseEvent)>::new(|event: MouseEvent| log!("Clicked"));
    //             onclick_ref
    //                 .get()
    //                 .expect("clickable element")
    //                 .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
    //                 .expect("add listener");
    //             || ()
    //         },
    //         onclick_ref,
    //     )
    // }

    // {
    //     let inner_host = inner_host.clone();
    //     use_effect_with_deps(
    //         |inner_host| {
    //             if let Some(ref inner_host) = **inner_host {
    //                 let div: HtmlElement = inner_host.clone().dyn_into().unwrap();
    //                 let inner = div.inner_html();
    //                 div.set_inner_html(&format!("<math>{inner}</math>"));
    //             }
    //         },
    //         inner_host,
    //     )
    // }

    {
        let host_ref = host_ref.clone();
        let ext_inner_host = inner_host.clone();
        use_effect_once(move || {
            let shadow_root = host_ref
                .get()
                .expect("rendered host")
                .unchecked_into::<Element>()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .expect("installating shadow root succeeds");
            let inner_host = gloo::utils::document()
                .create_element("div")
                .expect("can create inner wrapper");
            inner_host.set_inner_html("<math></math>");
            let math_inner_host: Element = inner_host.first_child().unwrap().dyn_into().unwrap();

            shadow_root
                .append_child(&inner_host)
                .expect("can attach inner host");
            //

            //
            ext_inner_host.set(Some(math_inner_host));

            || ()
        })
    }

    let contents = if let Some(ref inner_host) = *inner_host {
        let elem = gloo_utils::document().create_element("mrow").unwrap();
        elem.set_inner_html(
            "                    <mo>(</mo>\
                    <mtable>\
                    <mtr>\
                        <mtd {onclick}><mn>{1}</mn></mtd>\
                        <mtd><mn>{2}</mn></mtd>\
                        <mtd><mn>{3}</mn></mtd>\
                    </mtr>\
                    </mtable>\
                <mo>)</mo>",
        );
        create_portal(
            html! {
                VNode::VRef(elem.into())
            },
            inner_host.clone(),
        )
    } else {
        html! {}
    };

    html! {
        <div ref={host_ref.clone()}>
            {contents}
        </div>
    }
}

// <math>
//                 <mrow>
//     <mo>{"("}</mo>
//     <mtable>
//     <mtr>
//         <mtd {onclick}><mn>{1}</mn></mtd>
//         <mtd><mn>{2}</mn></mtd>
//         <mtd><mn>{3}</mn></mtd>
//     </mtr>
//     </mtable>
//     <mo>{")"}</mo>
// </mrow>
//                 </math>
