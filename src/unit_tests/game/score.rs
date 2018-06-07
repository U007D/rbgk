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
    expected_result: TestResult,
}

impl Env {
    fn new() -> Self {
        Self {
            game: Game::new(),
            result: TestResult::Untested,
            expected_result: TestResult::Untested,
        }
    }
}

#[test]
fn tests() {
    run(&given("a game", Env::new(), |ctx| {
        ctx.when("no balls are rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[]));
                env.expected_result = TestResult::Tested(None);
            });

            ctx.then("there is no score (score is None)", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("incomplete frame: a gutterball is rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[0]));
                env.expected_result = TestResult::Tested(Some(0));
            });

            ctx.then("the score is 0", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("incomplete frame: one pin is hit by one ball", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[1]));
                env.expected_result = TestResult::Tested(Some(1));
            });

            ctx.then("the score is 1", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("incomplete frame: two pins are hit by one ball", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[2]));
                env.expected_result = TestResult::Tested(Some(2));
            });

            ctx.then("the score is 2", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("open frame: two pins, then three pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[2, 3]));
                env.expected_result = TestResult::Tested(Some(5));
            });

            ctx.then("the score is 5", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("open frame: 4, 5 and then 6 pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[4, 5, 6]));
                env.expected_result = TestResult::Tested(Some(15));
            });

            ctx.then("the score is 15", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("spare: 4, 6 and then 5 pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[4, 5, 6]));
                env.expected_result = TestResult::Tested(Some(20));
            });

            ctx.then("the score is 20", |env| {
                assert!(env.result == env.expected_result);
            });
        });
    }));
}

