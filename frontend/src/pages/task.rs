use yew::prelude::*;

#[function_component(Task)]
pub fn task() -> Html {
    html! {
        <div>
            <h1>{ "task page" }</h1>
            <p>{ "Welcome to the task page." }</p>
            <p>{ "Here you can manage your tasks." }</p>
            //add task
            <input type="text" placeholder="Add a new task" />
            <button>{ "Add Task" }</button>

            //list of tasks
            <ul>
                <li>{ "Task 1" }</li>
                //delete task
                <button>{ "Delete" }</button>

                <li>{ "Task 2" }</li>
                 //delete task
                <button>{ "Delete" }</button>
                <li>{ "Task 3" }</li>
                 //delete task
                <button>{ "Delete" }</button>
            </ul>


        </div>
    }
}