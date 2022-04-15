use yew::prelude::*;

use crate::{constant::Status, context::gamestate_ctx::GameStateContext};

#[function_component(Statusline)]
pub fn statusline() -> Html {
    let state = use_context::<GameStateContext>().unwrap();

    let mode = {
        if state.status == Status::Playing {
            "INSERT".to_string()
        } else {
            "NORMAL".to_string()
        }
    };

    let mode_class = {
        if mode == "NORMAL" {
            "bg-green"
        } else {
            "bg-blue"
        }
    };

    html! {
        <>
            <div class="flex flex-row justify-between w-full bg-gray items-center font-bold text-white">
                <div class="flex flex-row gap-4 items-center">
                    <p class={classes!(String::from("text-black px-4 py-1"), mode_class)}>
                        {mode}
                    </p>
                    <div class="flex items-center gap-2">
                        <svg class="h-[18px]" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="clock" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256,8C119,8,8,119,8,256S119,504,256,504,504,393,504,256,393,8,256,8Zm92.49,313h0l-20,25a16,16,0,0,1-22.49,2.5h0l-67-49.72a40,40,0,0,1-15-31.23V112a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V256l58,42.5A16,16,0,0,1,348.49,321Z"></path></svg>
                        <p>{state.stats.time}</p>
                    </div>
                    <p>{format!("WPM: {}", state.stats.wpm)}</p>
                </div>
                <div class="flex flew-row items-center gap-4">
                    <p>{state.language.clone()}</p>
                    <p class={classes!(String::from("text-black px-4 py-1 min-w-[5rem] text-center"), mode_class)}>
                        {format!("{}%", state.stats.progress)}
                    </p>
                </div>
            </div>
        </>
    }
}
