// components/task_item.rs
use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo_timers::callback::Timeout;

#[function_component(TaskItem)]
pub fn task_item(task: &Task) -> Html {
    let title = use_state(|| task.title.clone());
    let debounce = use_state(|| None);

    let on_input = {
        let title = title.clone();
        let debounce = debounce.clone();
        let task_id = task.id.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let new_title = input.value();
            title.set(new_title.clone());

            // Cancel any pending timeout
            if let Some(timeout) = (*debounce).take() {
                timeout.cancel();
            }

            // Set a new timeout
            let task_id = task_id.clone();
            let debounce = debounce.clone();
            let timeout = Timeout::new(500, move || {
                // Send update to server and store optimistically
                let _ = api_client::update_task(task_id, new_title);
                debounce.set(None);
            });
            debounce.set(Some(timeout));
        })
    };

    html! {
        <input type="text" value={(*title).clone()} oninput={on_input} />
    }
}