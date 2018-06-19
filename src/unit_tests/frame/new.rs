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

            ctx.then("`Error::NoRolls` is returned", |env| {
                assert!(env.result == env.expected_result);
            });
        });
    }));
}

