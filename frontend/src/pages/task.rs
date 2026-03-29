use yew::prelude::*;

#[function_component(Task)]
pub fn task() -> Html {
    return html! {
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
                  //date and time
                <p>{ "Created at: 2024-06-01 10:00:00" }</p>
                <p>{ "This is the first task." }</p>
                //update task button
                <button>{ "Update" }</button>
                //delete task
                <button>{ "Delete" }</button>

                <li>{ "Task 2" }</li>
                    <p>{ "This is the second task." }</p>
                      //date and time
                <p>{ "Created at: 2024-06-01 10:00:00" }</p>
                     //update task button
                <button>{ "Update" }</button>
                 //delete task
                <button>{ "Delete" }</button>
                <li>{ "Task 3" }</li>
                    <p>{ "This is the third task." }</p>
                      //date and time
                <p>{ "Created at: 2024-06-01 10:00:00" }</p>
                     //update task button
                <button>{ "Update" }</button>
                 //delete task
                <button>{ "Delete" }</button>
            </ul>


        </div>
    }
}