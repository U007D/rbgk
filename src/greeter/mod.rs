mod hello_world_greeter;

pub use self::hello_world_greeter::HelloWorldGreeter;
pub use width_provider::{BitWidthProvider, WidthProvider};

pub trait Greeter {
    fn greet(&self, args: Vec<String>) -> String;
}
