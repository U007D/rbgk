use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HelloWorldGreeter<W: WidthProvider> {
    width_provider: W,
}

impl<W: WidthProvider> HelloWorldGreeter<W> {
    pub fn new(width_provider: W) -> Self {
        Self {
            width_provider,
        }
    }
}

impl<W: WidthProvider> Greeter for HelloWorldGreeter<W> {
    fn greet(&self, _args: Vec<String>) -> String {
        format!("Hello, {}-bit world!", self.width_provider.width())
    }
}
