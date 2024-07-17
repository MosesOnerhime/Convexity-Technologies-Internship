// main.rs
mod model;
mod view;
mod controller;

use crate::model::TaskManager;
use crate::controller::TaskController;
use crate::view::TaskView;

fn main() {
    let task_manager = TaskManager::new();
    let mut task_controller = TaskController::new(task_manager);

    loop {
        TaskView::display_message("\nWelcome to the To-Do List App!");
        TaskView::display_message("Choose an option:");
        TaskView::display_message("1. Add Task");
        TaskView::display_message("2. Complete Task");
        TaskView::display_message("3. List Tasks");
        TaskView::display_message("4. Exit");

        let choice: String = TaskView::prompt_input("\nEnter your choice:");

        match choice.as_str() {
            "1" => {
                let description = TaskView::prompt_input("Enter task description:");
                task_controller.add_task(description);
            }
            "2" => {
                let id_str = TaskView::prompt_input("Enter task ID to mark as complete:");
                if let Ok(id) = id_str.parse::<u32>() {
                    match task_controller.complete_task(id) {
                        Ok(_) => TaskView::display_message("Task marked as complete."),
                        Err(err) => TaskView::display_message(&err),
                    }
                } else {
                    TaskView::display_message("Invalid task ID.");
                }
            }
            "3" => {
                task_controller.list_tasks();
            }
            "4" => {
                TaskView::display_message("Exiting the application.");
                break;
            }
            _ => {
                TaskView::display_message("Invalid choice. Please choose again.");
            }
        }
    }
}
