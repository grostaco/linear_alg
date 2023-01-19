use yew::{function_component, html, Html};

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <>
        <nav class="navbar dflex-justify-center">
            <div class="dflex dflex-col">
                <div class="bold font-lg">{{"Linear Algebra Calculator"}}</div>
                <div>{{"Made with Rust!"}}</div>
            </div>
            <div class="dflex dflex-row dflex-gap-md">
                <a href="/">{"Home"}</a>
                <a href="https://github.com/grostaco">{"GitHub"}</a>
                <a href="https://grostaco.herokuapp.com/">{"About Me"}</a>
                <a href="https://github.com/grostaco/linear_alg">{"This site's code"}</a>
            </div>
        </nav>
        <div class="divider"></div>
        </>
    }
}
