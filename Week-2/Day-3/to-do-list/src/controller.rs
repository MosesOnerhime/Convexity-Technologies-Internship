use crate::model::TaskManager;

pub struct TaskController {
    task_manager: TaskManager,
}

impl TaskController {
    pub fn new(task_manager: TaskManager) -> Self {
        Self { task_manager }
    }

    pub fn add_task(&mut self, description: String) {
        self.task_manager.add_task(description);
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        self.task_manager.complete_task(id)
    }

    pub fn list_tasks(&self) {
        let tasks = self.task_manager.get_tasks();
        for task in tasks {
            println!("ID: {} - Description: {} - Completed: {}", task.id, task.description, task.completed);
        }
    }
}
