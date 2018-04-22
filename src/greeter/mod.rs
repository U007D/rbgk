mod width_provider;

pub use self::width_provider::WidthProvider;

pub trait Greeter {
    fn new() -> Self;
    fn greet(&self) -> String;
}
