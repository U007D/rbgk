use super::*;
use galvanic_test::*;
use galvanic_assert::matchers::*;
use galvanic_mock::{mockable, use_mocks};
//use galvanic_mock::*; //used in galvanic_mock_integration.rs example
//
//#[allow(redundant_closure, trivial_casts, unused_qualifications)] //does nothing; TODO: open issues
//test_suite! {
//    name app_tests;
//    use super::*;
//
//    fixture info_mock() -> mock::InfoMock {
//        setup(&mut self) {
//            // given
//            let arch_width = 42_usize;
//            let my_mock = new_mock!(Info for InfoMock);
//
//            given! {
//                bind arch_width: usize = arch_width;
//                <mock as Info>::width any_value() then_return bound.arch_width times 1;
//            }
//            my_mock
//        }
//    }
//
//    test yields_expected_word_width(info_mock) {
//        let expected_result = format!("Hello, {}-bit world!", arch_width);
//        let subject = App::new(info_mock.into_val());
//
//        // when
//        let result = subject.run();
//
//        // then
//        // the app returned the expected word width
//        assert_eq!(result.unwrap(), expected_result);
//    }
//}

test_suite! {
    name app_tests_wo_fixtures;
    use super::*;

    test yields_expected_word_width() {
        // given
        let arch_width = 42_usize;
        let mock = new_mock!(Info);

        given! {
            bind arch_width: usize = arch_width;
            <mock as Info>::width any_value() then_return bound.arch_width times 1;
        }

        let expected_result = format!("Hello, {}-bit world!", arch_width);
        let subject = App::new(mock);

        // when
        let result = subject.run();

        // then
        // the app returned the expected word width
        assert_eq!(result.unwrap(), expected_result);
    }
}
