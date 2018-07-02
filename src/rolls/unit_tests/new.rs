use super::*;

#[test]
fn no_roll_succeeds() {
    // given no rolls
    let rolls = &[];

    // the Roll is constructed
    let result = Rolls::new(rolls);

    // then the result should be `Ok()`
    assert!(result == Ok(Rolls(rolls.as_ref())));
}

#[test]
fn gutter_ball_succeeds() {
    // given a gutter ball
    let rolls = &[0];

    // the rolls are validated
    let result = Rolls::new(rolls);

    // then the result should be `Ok()`
    assert!(result == Ok(Rolls(rolls.as_ref())));
}

#[test]
fn roll_1_succeeds() {
    // given a roll of 1
    let rolls = &[1];

    // the rolls are validated
    let result = Rolls::new(rolls);

    // then the result should be `Ok()`
    assert!(result == Ok(Rolls(rolls.as_ref())));
}

#[test]
fn roll_10_succeeds() {
    // given a roll of 10
    let rolls = &[10];

    // the rolls are validated
    let result = Rolls::new(rolls);

    // then the score should be 10
    assert!(result == Ok(Rolls(rolls.as_ref())));
}

#[test]
fn roll_11_fails() {
    // given a roll of 11
    let rolls = &[11];

    // the rolls are validated
    let result = Rolls::new(rolls);

    // then the result should be an error indicating an invalid roll was provided
    assert!(result == Err(Error::InvalidRoll(*rolls.get(0).unwrap())));
}
