#[derive(PartialEq)]
pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}
#[derive(PartialEq)]
pub struct User {
    pub name: String,
    pub acessLevel: AccessLevel,
}

impl User {
    pub fn new(name: String, level: AccessLevel) -> User {
        Self { name, acessLevel: level }
    }
    pub fn send_name(&self) -> Option<&str> {
        if self.acessLevel == AccessLevel::Normal || self.acessLevel == AccessLevel::Admin {
            return Some(&self.name);
        }
        None
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    let res = match user.send_name() {
        Some(name) => (true, name),
        None => (false,"ERROR: User is guest"),
    };
    res
}
