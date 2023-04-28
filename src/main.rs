use yew::prelude::*;

#[function_component]
fn App() -> Html{
    // Text elements
    let title: String = String::from("Temperature Coverter");
    let text_celsius : String = "Celsius".to_string();
    let text_fahrenheit: String = String::from("Fahrenheit");
    let text_kelvin: String = String::from("Kelvin");

    // Input
    let input_c : f32 = 30.0;
    let input_f : f32 = 1.8 * input_c + 32.0;
    let input_k : f32 = input_c + 273.15;
    let str_result_c : String = input_c.to_string();
    let str_result_f : String = input_f.to_string();
    let str_result_k : String = input_k.to_string();

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
                    <p class="degree"> { str_result_c } </p>
                </div>
                <div class="flex-child">
                    <h2>
                        {text_fahrenheit}
                    </h2>
                    <p class="degree"> { str_result_f } </p>
                </div>
                <div class="flex-child">
                    <h2>
                        {text_kelvin}
                    </h2>
                    <p class="degree"> { str_result_k } </p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
