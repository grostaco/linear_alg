pub mod components;
mod routes;

fn main() {
    yew::Renderer::<routes::Home>::new().render();
}
