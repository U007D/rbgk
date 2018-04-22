use greeter::Greeter;
use width_provider::WidthProvider;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct HelloWorldGreeter<W: WidthProvider> {
    width_provider: W,
}

impl<W: WidthProvider> Greeter<W> for HelloWorldGreeter<W> {
    fn new(width_provider: W) -> Self {
        Self {
            width_provider,
        }
    }

    fn greet(&self) -> String {
        format!("Hello, {}-bit world!", self.width_provider.width())
    }
}
