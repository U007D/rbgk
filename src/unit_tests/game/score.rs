use super::*;

#[derive(Clone, Debug, PartialEq)]
enum TestResult {
    Untested,
    Tested(Result<u16>),
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
                env.expected_result = TestResult::Tested(Err(Error::NoRolls));
            });

            ctx.then("`Error::NoRolls` is returned)", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("a gutterball is rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[0]));
                env.expected_result = TestResult::Tested(Ok(0));
            });

            ctx.then("the score is 0", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("1 pin is hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[1]));
                env.expected_result = TestResult::Tested(Ok(1));
            });

            ctx.then("the score is 1", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("2 pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[2]));
                env.expected_result = TestResult::Tested(Ok(2));
            });

            ctx.then("the score is 2", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("3 then 4 pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[3, 4]));
                env.expected_result = TestResult::Tested(Ok(7));
            });

            ctx.then("the score is 7", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("a full game of open frames is rolled", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 2, 1, 2, 2]));
                env.expected_result = TestResult::Tested(Ok(51));
            });

            ctx.then("the score is 51", |env| {
                assert!(env.result == env.expected_result);
            });
        });

        ctx.when("an invalid number of pins are hit", |ctx| {
            ctx.before(|env| {
                env.result = TestResult::Tested(env.game.score(&[11]));
                env.expected_result = TestResult::Tested(Err(Error::InvalidRoll(11)));
            });

            ctx.then("an Error::InvalidRoll is returned", |env| {
                assert!(env.result == env.expected_result);
            });
        });
    }));
}

