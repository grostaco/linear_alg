use std::fmt::{Debug, Display};

use num_traits::{Float, FromPrimitive};
use yew::{function_component, html, virtual_dom::VNode, Html, Properties};

use crate::{
    components::Mat,
    math::{gauss::Step, GaussElimIter, Mat2d},
};

#[derive(Properties, PartialEq)]
pub struct Props<T, const M: usize, const N: usize>
where
    T: PartialEq,
{
    pub mat: Mat2d<T, M, N>,
    #[prop_or(M)]
    pub m: usize,
    #[prop_or(N)]
    pub n: usize,
}

#[function_component(Steps)]
pub fn steps<T, const M: usize, const N: usize>(props: &Props<T, M, N>) -> Html
where
    T: PartialEq + Copy + Float + Display + FromPrimitive + 'static + Debug,
{
    let mut iter = GaussElimIter::from(props.mat);
    let (m, n) = (props.m, props.n);
    iter.bound(m, n);

    let equations = gloo_utils::document()
        .create_element_ns(Some("http://www.w3.org/1998/Math/MathML"), "math")
        .unwrap();

    html! {
        <div class="dflex dflex-col dflex-gap-lg" style="margin-top: 2em;">
            {for iter.map(|(step, mat)| html! {
                <div class="dflex dflex-justify-center dflex-gap-md">
                    <Mat::<T, M, N> {m} {n} {mat}/>
                    <span>{
                        match step {
                            Step::Swap{from, to} => format!("Swap row {from} with row {to}"),
                            Step::Sub{scale, from, to} => format!("Mutliply row {from} by {scale} and subtract from row {to}"),
                        }
                    }</span>
                </div>
            } )}

            <p style="color: white">{"Solutions"}</p>
            {VNode::VRef(equations.into())}
        </div>
    }
}
