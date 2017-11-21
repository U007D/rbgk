use super::*;
use galvanic_test::*;
use galvanic_assert::matchers::*;
use galvanic_mock::{mockable, use_mocks};
use galvanic_mock::*; //used in galvanic_mock_integration.rs example

#[allow(redundant_closure, trivial_casts, unused_qualifications)] //does nothing; TODO: open issues
test_suite! {
    name app_tests;
    use super::*;

    fixture info_mock(arch_width: usize) -> mock::InfoMock {
        setup(&mut self) {
            let mock = new_mock!(Info for InfoMock);

            given! {
                bind arch_width: usize = *self.arch_width;
                <mock as Info>::width any_value() then_return bound.arch_width times 1;
            }
            mock
        }
    }

    test yields_expected_word_width(info_mock(42)) {
        // given
        let expected_result = format!("Hello, {}-bit world!", info_mock.params.arch_width);
        let subject = App::new(info_mock.into_val()).unwrap();

        // when
        let result = subject.run();

        // then
        // the app returned the expected word width
        assert_eq!(result.unwrap(), expected_result);
    }
}
