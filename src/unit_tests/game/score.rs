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

            ctx.then("there is no score (score is None", |env| {
                assert!(env.result == TestResult::Tested(None));
            });
        });
    }));
}

