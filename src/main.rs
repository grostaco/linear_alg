pub mod components;
mod math;
mod routes;

fn main() {
    yew::Renderer::<routes::Home>::new().render();
}
