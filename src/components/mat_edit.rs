use std::fmt::Display;

use num_traits::{identities, Float, FromPrimitive, ToPrimitive};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent};
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, use_state, Callback, Html,
    Properties, UseStateHandle,
};
use yew_hooks::use_effect_once;

use crate::math::Mat2d;

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + identities::Zero + Copy,
{
    pub mat: Mat2d<T>,
    pub onchange: Callback<Mat2d<T>, ()>,
}

#[function_component(MatEdit)]
pub fn mat_edit<T>(props: &Props<T>) -> Html
where
    T: Float + Display + 'static + FromPrimitive + ToPrimitive,
{
    let mat = {
        let mat = props.mat.clone();
        use_state(move || mat)
    };

    let matrix_ref = use_node_ref();
    let lparen_ref = use_node_ref();
    let rparen_ref = use_node_ref();

    let target: UseStateHandle<Option<HtmlElement>> = use_state(Option::default);

    let key = use_state(Option::default);

    let clear_target = use_state(bool::default);
    let nodes = use_state(Vec::<Vec<Element>>::default);

    let onclick = {
        let target_handle = target.clone();
        let key = key.clone();

        Callback::from(move |e: MouseEvent| {
            let element: HtmlElement = e.target().expect("click target").dyn_into().unwrap();
            if element.tag_name() != "SPAN" {
                return;
            }
            key.set(None);
            element.set_class_name("hasCursor");

            if let Some(ref target) = *target_handle {
                target.set_class_name("");
                if target == &element {
                    target_handle.set(None);
                    return;
                }
            }

            target_handle.set(Some(element));
        })
    };

    {
        let mat = mat.clone();
        use_effect_with_deps(
            move |new_mat| {
                let document = gloo_utils::document();
                mat.set(new_mat.clone());

                nodes.set(
                    new_mat
                        .iter()
                        .map(|v| {
                            v.iter()
                                .map(|x| {
                                    let node = document.create_element("span").unwrap();
                                    node.set_inner_html(&format!("{}", x));
                                    node
                                })
                                .collect()
                        })
                        .collect(),
                );
            },
            props.mat.clone(),
        );
    }

    {
        let key = key.clone();
        use_effect_once(move || {
            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                key.set(Some(event.key()));
            });
            gloo::utils::document()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            move || {
                gloo::utils::document()
                    .remove_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    }

    {
        let clear_target = clear_target.clone();
        let target = target.clone();
        use_effect_with_deps(
            move |clear_target| {
                if **clear_target {
                    if let Some(element) = target.as_ref() {
                        element.set_class_name("");
                        target.set(None);
                    }
                }
                clear_target.set(false);
            },
            clear_target,
        )
    }

    {
        use_effect_once(move || {
            let closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
                let element: HtmlElement =
                    event.target().expect("click target").dyn_into().unwrap();
                if element.tag_name() == "SPAN" {
                    return;
                }
                clear_target.set(true);
            });
            gloo::utils::document()
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .unwrap();
            move || {
                gloo::utils::document()
                    .remove_event_listener_with_callback(
                        "mousedown",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    }

    {
        let key = key.clone();
        let target = target.clone();
        let mat = mat.clone();
        let onchange = props.onchange.clone();

        use_effect_with_deps(
            move |(target, key)| {
                if let Some(key) = key.as_ref() {
                    gloo::console::log!(key.as_str());

                    if let Some(target) = target.as_ref() {
                        let i = target.get_attribute("i").expect("i index").parse().unwrap();
                        let j = target.get_attribute("j").expect("j index").parse().unwrap();

                        let mut mat_cloned = (*mat).clone();

                        match key.as_str() {
                            "Backspace" => {
                                mat_cloned[i][j] = T::from_u64(
                                    T::to_u64(
                                        &(mat[i][j] / T::from_f64(10.).expect("convert from f64")),
                                    )
                                    .expect("convert to u64"),
                                )
                                .expect("convert from u64");
                            }
                            // direction @ ("ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight") => {
                            //     let (ni, nj) = match direction {
                            //         "ArrowUp" => (i, j - 1),
                            //         "ArrowDown" => (i, j + 1),
                            //         "ArrowLeft" => (i - 1, j),
                            //         "ArrowRight" => (i + 1, j),
                            //         _ => (i, j),
                            //     };

                            // }
                            key => {
                                if let Ok(digit) = key.parse::<i64>() {
                                    mat_cloned[i][j] = mat[i][j]
                                        * T::from_f64(10.).expect("convert from f64")
                                        + T::from_i64(digit).expect("convert from i64");
                                    mat.set(mat_cloned.clone());
                                    onchange.emit(mat_cloned);
                                }
                            }
                        }
                    };
                }
                key.set(None);
            },
            (target, key),
        )
    }

    {
        let matrix_ref = matrix_ref.clone();
        let lparen_ref = lparen_ref.clone();
        let rparen_ref = rparen_ref.clone();

        use_effect_with_deps(
            move |(matrix_ref, _)| {
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
            (matrix_ref, props.mat.shape()),
        )
    }

    html! {
        <span class="block math">
            <span class="paren" ref={lparen_ref}>{"("}</span>
            <table class="matrix" ref={matrix_ref} {onclick}>
                <tbody>
                    {for (*mat).iter().enumerate().map(|(i, v)| html! {
                        <tr>
                            {for {v.iter().enumerate().map(|(j, v)| html! {
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
