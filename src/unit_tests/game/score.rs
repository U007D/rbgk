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

        ctx.when("a gutterball is rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[0]));
                env.expected_result = TestResult::Tested(Some(0));
            });

            ctx.then("the score is 0", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("1 pin is hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[1]));
                env.expected_result = TestResult::Tested(Some(1));
            });

            ctx.then("the score is 1", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("2 pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[2]));
                env.expected_result = TestResult::Tested(Some(2));
            });

            ctx.then("the score is 2", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("3 then 4 pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[3, 4]));
                env.expected_result = TestResult::Tested(Some(7));
            });

            ctx.then("the score is 7", |env| {
                assert!(env.result == env.expected_result);
            });
        });
    }));
}

