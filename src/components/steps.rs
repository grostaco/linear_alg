use std::fmt::{Debug, Display};

use num_traits::{Float, FromPrimitive};
use yew::{function_component, html, virtual_dom::VNode, Html, Properties};

use crate::{
    components::Mat,
    math::{mat::Step, Mat2d},
};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    pub mat: Mat2d<T>,
}

#[function_component(Steps)]
pub fn steps<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Copy + Float + Display + FromPrimitive + 'static + Debug,
{
    let rref = props.mat.rref();
    let steps = props.mat.row_reduced_verbose();

    let equations = gloo_utils::document()
        .create_element_ns(Some("http://www.w3.org/1998/Math/MathML"), "math")
        .unwrap();

    html! {
        <div class="dflex dflex-col dflex-gap-lg" style="margin-top: 2em;">
            {for steps.into_iter().map(|(mat, step)| html! {
                <div class="dflex dflex-justify-center dflex-gap-md">
                    <Mat::<T> {mat}/>
                    <span>{
                        match step {
                            Step::Swap{from, to} => format!("Swap row {from} with row {to}"),
                            Step::Sub{scale, from, to} => format!("Mutliply row {from} by {scale} and subtract from row {to}"),
                        }
                    }</span>
                </div>
            } )}


            <h1>{"RREF matrix"}</h1>
            <div class="dflex dflex-row dflex-justify-center dflex-gap-md">
                <Mat::<T> mat={rref}/>
            </div>
            {VNode::VRef(equations.into())}
        </div>
    }
}
