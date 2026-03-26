// components/task_list.rs
use yew_virtual_scroll::VirtualList;

#[function_component(TaskList)]
pub fn task_list(tasks: &Vec<Task>) -> Html {
    html! {
        <VirtualList
            items={tasks.clone()}
            height={600}
            item_height={50}
            render={Callback::from(|task: Task| html! { <TaskItem task={task} /> })}
        />
    }
}