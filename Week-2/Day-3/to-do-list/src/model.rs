// model.rs
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }
}

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.next_id += 1;
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", id))
        }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
