use width_provider::WidthProvider;

pub trait Greeter<W: WidthProvider> {
    fn new(width_provider: W) -> Self;
    fn greet(&self, _args: Vec<String>) -> String;
}
