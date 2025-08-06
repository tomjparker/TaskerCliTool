#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub cmd: String,
    pub depends_on: Vec<String>,
}

impl Task {
    pub fn run(&self) {
        println!("Running task: {} -> {}", self.name, self.cmd);
        // Real exec code goes here
    }
}