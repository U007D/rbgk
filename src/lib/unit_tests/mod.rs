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
        let mock = new_mock!(Info);

        given! {
            <mock as Info>::width(|_| true) then_return 42 times 1;
        }

        let expected_result = format!("Hello, {}-bit world!", arch_width);
        let app = App::new(mock);
    }
}
