
use crate::model::UserManager;
use crate::view::UserView;

pub struct UserController {
    manager: UserManager,
}

impl UserController {
    pub fn new(manager: UserManager) -> Self {
        Self { manager }
    }

    pub fn add_user(&mut self, name: String) {
        self.manager.add_user(name);
        UserView::display_message("User added successfully!");
    }

    pub fn valid_user(&mut self, id: u32) {
        self.manager.valid_user(id);
        UserView::display_message("User Valid!");
    }

    pub fn list_users(&self) {
        let users = self.manager.get_users();
        UserView::display_users(users);
    }
}
