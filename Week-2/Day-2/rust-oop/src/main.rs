// main.rs
mod model;
mod view;
mod controller;

use crate::model::TaskManager;
use crate::controller::TaskController;

fn main() {
    let task_manager = TaskManager::new();
    let mut controller = TaskController::new(task_manager);

    controller.add_task("Learn Rust".to_string());
    controller.add_task("Build a project".to_string());
    
    controller.list_tasks();
    
    controller.complete_task(1);
    
    controller.list_tasks();
}
