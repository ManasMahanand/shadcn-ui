use yew::prelude::*;

use crate::new_york::components::ui::textarea::Textarea;

#[function_component]
pub fn TextareaDemo() -> Html {
    html! {
        <Textarea placeholder="Type your message here." />
    }
}
