pub trait Greeter {
    fn new() -> Self;
    fn greet(&self) -> String;
}
