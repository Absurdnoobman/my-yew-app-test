use yew::prelude::*;

#[function_component]
fn App() -> Html{
    let allowfullscreen: bool = false;

    html! {
        <div>
            <iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ?controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" {allowfullscreen}></iframe>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
