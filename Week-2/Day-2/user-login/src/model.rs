
pub struct User {
    pub id: u32,
    pub name: String,
    pub valid: bool,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
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

    pub fn add_user(&mut self, name: String) {
        let id = (self.users.len() + 1) as u32;
        let user = User::new(id, name);
        self.users.push(user);
    }

    pub fn valid_user(&mut self, id: u32) {
        if let Some(user) = self.users.iter_mut().find(|t| t.id == id) {
            user.valid = true;
        }
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
}