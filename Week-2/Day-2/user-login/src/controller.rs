
use crate::model::UserManager;
use crate::view::UserView;

pub struct UserController {
    manager: UserManager,
}

impl UserController {
    pub fn new(manager: UserManager) -> Self {
        Self { manager }
    }

    pub fn add_user(&mut self, name: &str) {
        self.manager.add_user(name);
        UserView::display_message("User added successfully!");
    }

    pub fn valid_user(&mut self, name: &str) -> bool{
        if let Some(user) = self.manager.get_user_by_name(name) {
            if user.valid {
                UserView::display_message("Error: User is not valid!");
                false
            } else {
                self.manager.valid_user(name);
                UserView::display_message("User valid!");
                true
                
            }
        } else {
            UserView::display_message("\nUsername not valid.");
            false
            
        }
    }

    pub fn list_users(&self) {
        let users = self.manager.get_users();
        UserView::display_users(users);
    }
}
