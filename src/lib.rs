use wasm_bindgen::prelude::*;
use yew::prelude::*;

// This sets up the 'root' component called Model which will render a button that
// updates on click.
// SUS OUT App::<Model>::new().mount_to_body() - This,
// Starts the app and ounts it to the body tag.
// If I want to attach any dynamic properties, I can use
// mount_to_body_with_props(...)
// Data?
struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    // Creating properties?
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return true if new properties are different to
        // the previously recieved properties.
        // This component has no properties so we will always return "false".
        false
    }
    // Renderer?
    fn view(&self) -> Html {
        // BANG
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{self.value}</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
