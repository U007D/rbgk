use super::*;
use galvanic_test::*;
#[allow(unused_imports)] use galvanic_assert::matchers::*;
use concurrency_primitives::APP_ALREADY_INITIALIZED;

test_suite! {
    name concurrency_primitives;
    use super::*;
//    fixture singleton_mock() -> mock::AppStateMock {
//        setup(&mut self) {
//            let mock = new_mock!(Singleton for AppStateMock);
//
//            given! {
//                <mock as Singleton>::{
//                    already_instantiated() then_return false times 1;
//                    already_instantiated() then_return true times always;
//                }
//                mock
//            }
//        }
//    }

    fixture app_state_obj() -> AppState {
        setup(&mut self) {
        // Set atomic backing store to uninitialized
            AppState::new()
        }

        tear_down(&self) {
            // Reset atomic backing store to uninitialized
            APP_ALREADY_INITIALIZED.compare_and_swap(true, false, Ordering::Relaxed);
        }
    }

    test first_call_indicates_uninitialized(app_state_obj()) {
        // given
        let expected_result = false;
        let subject = app_state_obj.into_val();

        // when
        let result = subject.already_initialized();

        // then
        assert_eq!(result, expected_result);
    }
}
