// view.rs
use crate::model::Task;

pub struct TaskView;

impl TaskView {
    pub fn display_tasks(tasks: &Vec<Task>) {
        for task in tasks {
            println!(
                "ID: {} - Description: {} - Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }

    pub fn display_message(message: &str) {
        println!("{}", message);
    }
}
