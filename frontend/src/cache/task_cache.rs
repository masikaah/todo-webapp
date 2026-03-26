// cache/task_cache.rs
use gloo::storage::{IndexedDb, Storage};
use shared::models::Task;

const TASKS_STORE: &str = "tasks";

pub async fn load_cached_tasks() -> Result<Vec<Task>, gloo::storage::errors::StorageError> {
    let db = IndexedDb::new("taskflow_cache")?;
    db.open()?;
    let tasks: Vec<Task> = db.get(TASKS_STORE).await?;
    Ok(tasks)
}

pub async fn save_tasks(tasks: &[Task]) -> Result<(), gloo::storage::errors::StorageError> {
    let db = IndexedDb::new("taskflow_cache")?;
    db.open()?;
    db.set(TASKS_STORE, tasks).await?;
    Ok(())
}