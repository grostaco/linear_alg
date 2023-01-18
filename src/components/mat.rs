use std::fmt::Display;

use num_traits::{identities, Float, FromPrimitive, ToPrimitive};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use yew::{function_component, html, use_effect_with_deps, use_node_ref, Html, Properties};

use crate::math::Mat2d;

#[derive(Properties, PartialEq)]
pub struct Props<T, const M: usize, const N: usize>
where
    T: PartialEq + identities::Zero + Copy,
{
    #[prop_or(Mat2d::zeros())]
    pub mat: Mat2d<T, M, N>,
    #[prop_or(M)]
    pub m: usize,
    #[prop_or(N)]
    pub n: usize,
}
#[function_component(Mat)]
pub fn mat<T, const M: usize, const N: usize>(props: &Props<T, M, N>) -> Html
where
    T: Float + Display + 'static + FromPrimitive + ToPrimitive,
{
    let matrix_ref = use_node_ref();
    let lparen_ref = use_node_ref();
    let rparen_ref = use_node_ref();

    {
        let matrix_ref = matrix_ref.clone();
        let lparen_ref = lparen_ref.clone();
        let rparen_ref = rparen_ref.clone();

        use_effect_with_deps(
            move |(matrix_ref, _, _)| {
                let element: Element = matrix_ref
                    .clone()
                    .get()
                    .expect("attached node")
                    .dyn_into()
                    .expect("html element");
                let scale = element.client_height() as f64 / 20. * 1.25;
                let lparen: HtmlElement = lparen_ref
                    .get()
                    .expect("left parenthesis node")
                    .dyn_into()
                    .unwrap();
                let rparen: HtmlElement = rparen_ref
                    .get()
                    .expect("right parenthesis node")
                    .dyn_into()
                    .unwrap();

                lparen
                    .set_attribute("style", &format!("transform: scale(1.2, {scale})"))
                    .expect("set scale attribute");
                rparen
                    .set_attribute("style", &format!("transform: scale(1.2, {scale})"))
                    .expect("set scale attribute");
            },
            (matrix_ref, props.m, props.n),
        )
    }

    html! {
        <span class="block math">
            <span class="paren" ref={lparen_ref}>{"("}</span>
            <table class="matrix" ref={matrix_ref}>
                <tbody>
                    {for props.mat.iter().take(props.m).enumerate().map(|(i, v)| html! {
                        <tr>
                            {for {v.iter().take(props.n).enumerate().map(|(j, v)| html! {
                                <td>
                                    <span i={i.to_string()} j={j.to_string()}>{v}</span>
                                </td>
                            })} }
                        </tr>
                    })}
                </tbody>
            </table>
            <span class="paren" ref={rparen_ref}>{")"}</span>
        </span>
    }
}
