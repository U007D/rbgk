extern crate hesl;

mod mock_greeter;

use hesl::macros::assert;
use rspec::{given, run};
#[allow(unused_imports)]
use super::*;

#[test]
fn tests() {
    run(&given("a di container configured to inject a mock greeter", (), |ctx| {
        let container = TestGreeterContainer::build();
        let expected_result = String::from("Hello, this is a test greeting.");
        let greet_expected_times_called = 1;

        ctx.when("resolved and greet() is called", |ctx| {
            let greeter = container.resolve_greeter();
            let result = greeter.greet();

            ctx.then("the mock greeter should show that greet() was called exactly once", |_| {
                assert!(greeter.greet_times_recalled == greet_expected_times_called);
            });

            ctx.then("the expected greeting should be received", |_| {
                assert!(result == expected_result);
            });
        });
    }));
}

