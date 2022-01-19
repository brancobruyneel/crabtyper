use yew::prelude::*;

#[function_component(LineNumber)]
pub fn line_number() -> Html {
    html! {
        <div class="text-white">
            <p>{"1"}</p>
            <p>{"2"}</p>
            <p>{"3"}</p>
            <p>{"4"}</p>
            <p>{"5"}</p>
            <p>{"6"}</p>
            <p>{"7"}</p>
        </div>
    }
}
