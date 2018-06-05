use super::*;

#[derive(Clone, Debug, PartialEq)]
enum TestResult {
    Untested,
    Tested(Option<u16>),
}

impl Default for TestResult {
    fn default() -> Self {
        TestResult::Untested
    }
}

#[derive(Clone, Default, Debug)]
struct Env {
    game: Game,
    result: TestResult,
}

impl Env {
    fn new() -> Self {
        Self {
            game: Game::new(),
            result: TestResult::Untested,
        }
    }
}

#[test]
fn tests() {
    run(&given("a game", Env::default(), |ctx| {
        ctx.when("no balls are rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[]));
            });

            ctx.then("there is no score (score is None)", |env| {
                assert!(env.result == TestResult::Tested(None));
            });
        });

        ctx.when("a gutterball is rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[0]));
            });

            ctx.then("the score is 0", |env| {
                assert!(env.result == TestResult::Tested(Some(0)));
            });
        });

        ctx.when("one pin is hit by one ball", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[1]));
            });

            ctx.then("the score is 1", |env| {
                assert!(env.result == TestResult::Tested(Some(1)));
            });
        });

        ctx.when("two pins are hit by one ball", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[2]));
            });

            ctx.then("the score is 2", |env| {
                assert!(env.result == TestResult::Tested(Some(2)));
            });
        });

    }));
}

