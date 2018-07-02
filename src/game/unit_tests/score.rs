use super::*;

#[test]
fn no_roll_yields_none() {
    // given a game instance
    let game = Game::new();

    // when a score is calculated when no balls have been rolled
    let result = game.score(&Rolls::new(&[]).unwrap());

    // then the result should be `None`
    assert!(result == Ok(None));
}

#[test]
fn gutter_ball_yields_0() {
    // given a game instance
    let game = Game::new();

    // when a score is calculated when a gutter ball has been rolled
    let result = game.score(&Rolls::new(&[0]).unwrap());

    // then the score should be 0
    assert!(result == Ok(Some(0)));
}

#[test]
fn roll_1_yields_1() {
    // given a game instance
    let game = Game::new();

    // when a score is calculated when a roll hits one pin
    let result = game.score(&Rolls::new(&[1]).unwrap());

    // then the score should be 1
    assert!(result == Ok(Some(1_u16)));
}

#[test]
fn roll_9_yields_9() {
    // given a game instance
    let game = Game::new();

    // when a score is calculated when a roll hits 9 pins
    let result = game.score(&Rolls::new(&[9]).unwrap());

    // then the score should be 9
    assert!(result == Ok(Some(9)));
}

#[test]
fn roll_10_yields_10() {
    // given a game instance
    let game = Game::new();

    // when a score is calculated when a roll hits 10 pins
    let result = game.score(&Rolls::new(&[10]).unwrap());

    // then the score should be 10
    assert!(result == Ok(Some(10_u16)));
}

#[test]
fn roll_all_4s_yields_80() {
    // given a game instance
    let game = Game::new();

    // when a score is calculated with a full game (10 frames) of 4-rolls (2 rolls per frame)
    let result = game.score(&Rolls::new(&[4; 20]).unwrap());

    // then the score should be 80
    assert!(result == Ok(Some(80_u16)));
}
