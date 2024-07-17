
pub struct User {
    pub id: u32,
    pub name: String,
    pub valid: bool,
}

impl User {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            valid: false,
        }
    }
}

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_user(&mut self, name: &str) {
        let id = (self.users.len() + 1) as u32;
        let user = User::new(id, name);
        self.users.push(user);
    }

    pub fn valid_user(&mut self, name: &str) {
        let user_option = self.users.iter_mut().find(|t| t.name == name);
        
        match user_option {
            Some(user) => {
                user.valid = true;
            }
            None => {
                // Iterate over all users and set `valid` to false
                for user in &mut self.users {
                    user.valid = false;
                }
            }
        }
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|&t| t.id == id)
    }
    
    pub fn get_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|&t| t.name == name)
    }

}