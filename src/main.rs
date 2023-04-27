use yew::prelude::*;

#[function_component]
fn App() -> Html{
    let h_one : String = String::from("Helloworld");

    html! {
        <div>
            <h1 name="hi" style="color:red;"> {h_one} </h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
