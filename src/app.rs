use yew::prelude::*;
use crate::handle_command::handle_command;

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(|| "".to_string());
    let output = use_state(|| "".to_string());

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            input.set(value);
        })
    };

    let onsubmit = {
        let input = input.clone();
        let output = output.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let res = handle_command(&*input);
            output.set(res);
        })
    };

    html! {
        <div style="font-family: monospace; padding: 1rem;">
            <h1>{"Rust Excel in Browser"}</h1>
            <form onsubmit={onsubmit}>
                <input value={(*input).clone()} {oninput} placeholder="Enter command..." />
                <button type="submit">{"Run"}</button>
            </form>
            <pre>{ (*output).clone() }</pre>
        </div>
    }
}
