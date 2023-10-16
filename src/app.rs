mod tauri_extern;

use tauri_extern::invoke;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let handle_click_open_directory = Callback::from(move |_| {
        spawn_local(async move {
            let _ = invoke("pick_folder", JsValue::UNDEFINED).await;
        });
    });

    html! {
        <main class={classes!("bg-red-600")}>
            <button onclick={handle_click_open_directory}>{"Open directory"}</button>
        </main>
    }
}
