use rspec::{given, run};
#[allow_unused]
use super::*;

#[test]
fn tests() {
    run(&given("", (), |ctx| {
        ctx.when("", |ctx| {
            ctx.then("", |_| {
                assert!();
            });

            ctx.then("", |_| {
                assert!();
            });
        });
    }));
}

