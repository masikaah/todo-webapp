use yew::prelude::*;
use crate::cache::task_cache;
use crate::store::tasks::{TaskState, TaskAction};

#[function_component(Home)]
pub fn home() -> Html {
    let tasks_state = use_reducer(TaskState::default);
    let tasks = tasks_state.tasks.clone();

    use_effect_with_deps(move |_| {
        // 1. Immediately load from cache and populate store
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(cached) = task_cache::load_cached_tasks().await {
                tasks_state.dispatch(TaskAction::SetAll(cached)); // custom action
            }
        });

        // 2. Fetch fresh data from API and update store + cache
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(fresh) = api_client::get_tasks().await {
                tasks_state.dispatch(TaskAction::SetAll(fresh.clone()));
                let _ = task_cache::save_tasks(&fresh).await;
            }
        });
    }, ());

    // render tasks using virtual list
    html! { <TaskList tasks={tasks} /> }
}