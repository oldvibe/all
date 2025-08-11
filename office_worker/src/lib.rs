#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let s = s.split(",").collect::<Vec<_>>();
        println!("s     >>>>> {:?}", s);
        OfficeWorker{
            name : s[0].to_string(),
            age: s[1].parse().unwrap(),
            role: s[2].into()
        }
    }
}

impl From<&str> for WorkerRole {
    fn from(r: &str) -> Self {
        match r {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            _ => WorkerRole::Guest,
        }
    }
}
