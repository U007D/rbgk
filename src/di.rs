use super::*;

pub trait ContainerConfig {
    type Greeter: Greeter<Self::WidthProvider> = HelloWorldGreeter<Self::WidthProvider>;
    type WidthProvider: WidthProvider = UniversalWidthProvider;
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct Container {}

impl ContainerConfig for Container {}

impl Container {

    pub fn resolve_greeter() -> <Self as ContainerConfig>::Greeter {
        <Self as ContainerConfig>::Greeter::new(Self::resolve_width_provider())
    }

    pub fn resolve_width_provider() -> <Self as ContainerConfig>::WidthProvider {
        <Self as ContainerConfig>::WidthProvider::new()
    }

}
