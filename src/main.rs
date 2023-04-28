use yew::prelude::*;

#[function_component]
fn App() -> Html{
    // Text elements
    let title: String = String::from("Temperature Coverter");
    let text_celsius : String = "Celsius".to_string();
    let text_fahrenheit: String = String::from("Fahrenheit");
    let text_kelvin: String = String::from("Kelvin");

    // Input
    let input_c : i32 = 30;
    let str_input_c : String = input_c.to_string();

    html! {
        <div style="text-align: center;">
            <div style="background-color: DodgerBlue; padding: 5px">
                <h1 style="color: White;"> { title } </h1>
            </div>
            <div class="flex-container">
                <div class="flex-child">
                    <h2>
                        {text_celsius}
                    </h2>
                    <p> {str_input_c} </p>
                </div>
                <div class="flex-child">
                    <h2>
                        {text_fahrenheit}
                    </h2>
                </div>
                <div class="flex-child">
                    <h2>
                        {text_kelvin}
                    </h2>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
