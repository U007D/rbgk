extern crate hesl;

mod mock_greeter;
mod test_greeter_container;

use hesl::macros::assert;
use rspec::{given, run};
use self::test_greeter_container::TestGreeterContainer;
#[allow(unused_imports)]
use super::*;
use std::sync::atomic::Ordering;

#[test]
fn tests() {
    run(&given("a di container configured to inject a mock greeter", (), |ctx| {
        let container = TestGreeterContainer::build();
        let expected_result = String::from("Hello, this is a test greeting.");
        let greet_expected_times_called = 1;

        ctx.when("resolved and greet() is called", |ctx| {
            let greeter = container.resolve_greeter();
            let result = greeter.greet();

            ctx.then("the mock greeter should show that greet() was called exactly once", move |_| {
                assert!(greeter.greet_times_called.load(Ordering::Relaxed) == greet_expected_times_called);
            });

            ctx.then("the expected greeting should be received", move |_| {
                assert!(result == expected_result);
            });
        });
    }));
}

