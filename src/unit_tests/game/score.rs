use super::*;

#[derive(Clone, Default, Debug)]
struct Env {
    game: Game,
    result: Option<Option<u16>>,
}

impl Env {
    fn new() -> Self {
        Self {
            game: Game::new(),
            result: None,
        }
    }
}

#[test]
fn tests() {
    run(&given("a game", Env::default(), |ctx| {
        ctx.when("no balls are rolled", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.game.score(&[]));
            });

            ctx.then("the score will be None", |env| {
                assert!(env.game.score() == Some(None));
            });
        });
    }));
}

