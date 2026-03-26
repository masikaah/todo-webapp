// store/tasks.rs
use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use shared::models::Task;

#[derive(Clone, PartialEq)]
pub struct TaskState {
    pub tasks: Vec<Task>,
    pub pending_operations: Vec<Operation>, // track optimistic updates
}

pub enum TaskAction {
    AddOptimistic(Task),
    AddSuccess(Task),
    AddFailed(Task), // revert
    // ... similar for update, delete
}

impl Reducible for TaskState {
    type Action = TaskAction;

    fn reduce(self, action: Self::Action) -> Self {
        match action {
            TaskAction::AddOptimistic(task) => {
                let mut tasks = self.tasks;
                tasks.push(task);
                Self { tasks, ..self }
            }
            TaskAction::AddSuccess(task) => {
                // replace optimistic task with server-confirmed version
                // (usually same, but server may add id, timestamps)
                let tasks = self.tasks
                    .iter()
                    .map(|t| if t.client_id == task.client_id { task.clone() } else { t.clone() })
                    .collect();
                Self { tasks, ..self }
            }
            TaskAction::AddFailed(original) => {
                // remove the optimistic task
                let tasks = self.tasks
                    .into_iter()
                    .filter(|t| t.client_id != original.client_id)
                    .collect();
                Self { tasks, ..self }
            }
        }
    }
}

// store/tasks.rs
pub enum SyncMessage {
    TaskUpdated(Task),
    TaskDeleted(String),
    TaskCreated(Task),
}

impl TaskState {
    pub fn apply_sync(&mut self, msg: SyncMessage) {
        match msg {
            SyncMessage::TaskUpdated(task) => {
                // update or insert
                if let Some(pos) = self.tasks.iter().position(|t| t.id == task.id) {
                    self.tasks[pos] = task;
                } else {
                    self.tasks.push(task);
                }
            }
            SyncMessage::TaskDeleted(id) => {
                self.tasks.retain(|t| t.id != id);
            }
            SyncMessage::TaskCreated(task) => {
                self.tasks.push(task);
            }
        }
        // also update cache
    }
}