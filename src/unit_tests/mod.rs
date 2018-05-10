extern crate hesl;

use rspec::{given, run};
use di::Container;

#[test]
fn tests() {
    run(&given("", (), |ctx| {

        ctx.when("", |ctx| {

            ctx.then("", move |_| {
            });
        });
    }));
}

