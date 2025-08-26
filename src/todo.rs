pub struct ToDo(bool, pub String);

impl ToDo {
    pub fn new(status: bool, task: &str) -> Self {
        ToDo(status, task.to_string())
    }

    pub fn check(&mut self) {
        self.0 = true;
    }
    
    pub fn is_done(&self) -> bool {
        self.0
    }
}