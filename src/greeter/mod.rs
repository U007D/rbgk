mod hello_world_greeter;
mod width_provider;

pub use self::hello_world_greeter::HelloWorldGreeter;
pub use self::width_provider::{BitWidthProvider, WidthProvider};

pub trait Greeter {
    fn greet(&self, args: Vec<String>) -> String;
}
