pub trait Task {}
pub struct FibTask {}
impl Task for FibTask {}

pub trait Registry {
    fn register(&mut self);
}

#[derive(Default)]
pub struct EngineAPI {
    task: Option<Box<dyn Task>>,
}

impl Registry for EngineAPI {
    fn register(&mut self) {
		// removing this segfaults???
        println!("cursed");
        self.task = Some(Box::new(FibTask {}));
    }
}
