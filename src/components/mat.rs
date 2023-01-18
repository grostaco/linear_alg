use std::fmt::Display;

use num_traits::identities;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent};
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, use_state, Callback, Html,
    Properties, UseStateHandle,
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
    #[prop_or(M)]
    pub m: usize,
    #[prop_or(N)]
    pub n: usize,
}
#[function_component(Mat)]
pub fn mat<T, const M: usize, const N: usize>(props: &Props<T, M, N>) -> Html
where
    T: PartialEq + identities::Zero + Copy + Display,
{
    let matrix_ref = use_node_ref();
    let lparen_ref = use_node_ref();
    let rparen_ref = use_node_ref();

    let target: UseStateHandle<Option<HtmlElement>> = use_state(Option::default);

    let key = use_state(Option::default);

    let clear_target = use_state(bool::default);

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
        use_effect_with_deps(
            move |(target, key)| {
                if let Some(key) = key.as_ref() {
                    gloo::console::log!(key.as_str());

                    if let Some(target) = target.as_ref() {
                        let inner = target.inner_html();
                        if key == "Backspace" {
                            if inner.len() > 1 {
                                target.set_inner_html(
                                    inner.get(..inner.len() - 1).expect("string slice"),
                                );
                            } else {
                                target.set_inner_html("0");
                            }
                        }

                        if (key.chars().nth(0).unwrap_or('A') as u8)
                            .checked_sub('0' as u8)
                            .map(|x| x <= 9)
                            .unwrap_or(false)
                        {
                            if inner != "0" {
                                target.set_inner_html(&(inner + &key));
                            } else {
                                target.set_inner_html(key);
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
            <table class="matrix" ref={matrix_ref} {onclick}>
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
