extern crate hesl;

mod mocks;
mod test_containers;

use rspec::{given, run};
use di::Container;
use self::test_containers::{TestGreeterContainer, TestWidthProviderContainer};
use self::greeter::{Greeter, WidthProvider};
use std::sync::atomic::Ordering;
use super::*;

#[test]
fn tests() {
    run(&given("a GreeterContainer configured to inject a MockGreeter", (), |ctx| {
        let container = TestGreeterContainer::build();
        let expected_result = String::from("Hello, this is a test greeting.");
        let greet_expected_times_called = 1;

        ctx.when("the Greeter is resolved and greet() is called", |ctx| {
            let greeter = container.resolve_greeter();
            let result = greeter.greet(Vec::new());

            ctx.then("the MockGreeter should show that greet() was called exactly once", move |_| {
                assert!(greeter.greet_times_called.load(Ordering::Relaxed) == greet_expected_times_called);
            });

            ctx.then("the expected greeting should be received", move |_| {
                assert!(result == expected_result);
            });
        });
    }));

    run(&given("a GreeterContainer configured to inject a MockWidthProvider", (), |ctx| {
        let container = TestWidthProviderContainer::build();
        let expected_result = 42;
        let width_expected_times_called = 1;

        ctx.when("the WidthProvider resolved and width() is called", |ctx| {
            let width_provider = container.resolve_width_provider();
            let result = width_provider.width();

            ctx.then("the MockWidthProvider should show that width() was called exactly once", move |_| {
                assert!(width_provider.width_times_called.load(Ordering::Relaxed) == width_expected_times_called);
            });

            ctx.then("the expected value should be received", move |_| {
                assert!(result == expected_result);
            });
        });
    }));

    run(&given("a GreeterContainer configured to inject a HelloWorldGreeter", (), |ctx| {
        let container = GreeterContainer::build();
        #[cfg(target_pointer_width = "32")]
        let expected_result = "Hello, 32-bit world!";
        #[cfg(target_pointer_width = "64")]
        let expected_result = "Hello, 64-bit world!";

        ctx.when("the Greeter is resolved and greet() is called", |ctx| {
            let result = container.resolve_greeter()
                                  .greet(Vec::new());

            ctx.then("the expected value should be received", move |_| {
                assert!(result == expected_result);
            });
        });
    }));
}

