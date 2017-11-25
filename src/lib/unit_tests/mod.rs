use super::*;
use galvanic_test::*;
use galvanic_assert::matchers::*;   // WORKAROUND for bug https://github.com/mindsbackyard/galvanic-mock/issues/1
use concurrency_primitives::Singleton;

test_suite! {
    name app_info;
    use super::*;

    fixture app_state_mock() -> mock::AppStateMock {
        setup(&mut self) {
            let mock = new_mock!(Singleton for AppStateMock);

            given! {
                <mock as Singleton>::{
                    already_instantiated any_value() then_return false times 1;
                    already_instantiated any_value() then_return true always;
                }
            }
            mock
        }
    }

    fixture arch_mock(arch_width: usize) -> mock::ArchMock {
        setup(&mut self) {
            let mock = new_mock!(Info for ArchMock);

            given! {
                bind arch_width: usize = *self.arch_width;
                <mock as Info>::width any_value() then_return bound.arch_width times 1;
            }
            mock
        }
    }

    test yields_expected_word_width(app_state_mock(), arch_mock(42)) {
        // given
        let expected_result = format!("Hello, {}-bit world!", arch_mock.params.arch_width);
        let subject = App::new(&app_state_mock.val, arch_mock.val).unwrap();

        // when
        let result = subject.run();

        // then
        // the app returned the expected word width
        assert_eq!(result.unwrap(), expected_result);
    }
}
