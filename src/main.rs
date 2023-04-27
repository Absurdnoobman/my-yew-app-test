use yew::prelude::*;

#[function_component]
fn App() -> Html{
    let h_one : String = String::from("Hello world");

    html! {
        <div>
            <h1 name="hi" title="Hi i'm kolo"> {h_one} </h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
