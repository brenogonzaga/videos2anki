use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let on_click = Callback::from(move |_| {
        let _ = spawn_local(async {
            let _ = invoke("run", JsValue::NULL).await;
        });
    });

    html! {
        <main class="container">
            <button onclick={on_click}>{"Run"}</button>
        </main>
    }
}
