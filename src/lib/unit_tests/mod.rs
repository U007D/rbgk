use super::*;
use galvanic_test::*;
use galvanic_assert::matchers::*;
use galvanic_mock::{mockable, use_mocks};

test_suite! {
    name app_tests;
    use super::*;

    test yields_expected_word_width() {
        //given
        let arch_width = 42_usize;
        let expected_result = format!("Hello, {}-bit world!\n", arch_width);
        let mut app = App::new(|| arch_width);

        //when
        let result = app.run();

        //then
        assert_eq!(result.unwrap(), expected_result)
    }
}
