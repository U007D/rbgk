use di::Container;
use self::mocks::MockWidthProvider;
use super::*;

pub struct TestWidthProviderContainer {}

impl Container for TestWidthProviderContainer {
    fn build() -> Self {
        Self {}
    }
}

impl TestWidthProviderContainer {
    pub fn resolve_width_provider(&self) -> MockWidthProvider {
        MockWidthProvider::new()
    }
}
