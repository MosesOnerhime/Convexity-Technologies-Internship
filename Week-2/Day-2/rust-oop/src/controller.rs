use crate::model::TaskManager;
use crate::view::TaskView;

pub struct TaskController {
    manager: TaskManager,
}

impl TaskController {
    pub fn new(manager: TaskManager) -> Self {
        Self { manager }
    }

    pub fn add_task(&mut self, description: String) {
        self.manager.add_task(description);
        TaskView::display_message("Task added successfully!");
    }

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.manager.get_task_by_id(id) {
            if task.completed {
                TaskView::display_message("Error: Task is already completed!");
            } else {
                self.manager.complete_task(id);
                TaskView::display_message("Task completed!");
            }
        } else {
            TaskView::display_message("Error: Task not found!");
        }
    }

    pub fn list_tasks(&self) {
        let tasks = self.manager.get_tasks();
        TaskView::display_tasks(tasks);
    }
}
