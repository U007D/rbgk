use self::mock_greeter::MockGreeter;
use super::*;

pub struct TestGreeterContainer {}

impl DiContainer for TestGreeterContainer {
    fn build() -> Self {
        Self {}
    }
}

impl TestGreeterContainer {
    pub fn resolve_greeter(&self) -> MockGreeter {
        MockGreeter::new()
    }
}
