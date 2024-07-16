
mod model;
mod view;
mod controller;

use crate::model::UserManager;
use crate::controller::UserController;

fn main() {
    let user_manager = UserManager::new();
    let mut controller = UserController::new(user_manager);

    controller.add_user("Moses Onerhime".to_string());
    controller.add_user("Micheal Dowan".to_string());
    
    controller.list_users();
    
    controller.valid_user(1);
    controller.valid_user(4);
    
    controller.list_users();
}
