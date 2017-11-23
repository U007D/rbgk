use super::*;
use galvanic_test::*;
#[allow(unused_imports)] use galvanic_assert::matchers::*;
use concurrency_primitives::APP_ALREADY_INITIALIZED;
use std::thread;

test_suite! {
    name app_state;
    use super::*;

    fixture app_state() -> AppState {
        setup(&mut self) {
            AppState::new()
        }

        tear_down(&self) {
            // Reset 'static atomic backing store to uninitialized to enable use in multiple tests
            APP_ALREADY_INITIALIZED.compare_and_swap(true, false, Ordering::SeqCst);
        }
    }

    test single_threaded_first_call_yield_uninitialized(app_state()) {
        // given
        let subject = app_state.val;

        // then when
        assert_eq!(subject.already_initialized(), false);
    }

    test single_threaded_calls_after_first_yield_initialized(app_state()) {
        // given
        let subject = app_state.val;
        assert_eq!(subject.already_initialized(), false);

        // then when
        assert_eq!(subject.already_initialized(), true);
        assert_eq!(subject.already_initialized(), true);
        assert_eq!(subject.already_initialized(), true);
    }

    test multi_threaded_yields_uninitialized_exactly_once(app_state()) {
        // given
        const N_THREADS: usize = 8;
        let subject = app_state.val;

        // when
        let results = (0..N_THREADS).map(|_| subject.clone())
                                    .map(|sut| thread::spawn(move || -> bool {
                                            sut.already_initialized()
                                        }))
                                    .collect::<Vec<_>>()  // Spawn threads asynchronously
                                    .into_iter()
                                    .map(|h| h.join().unwrap())
                                    .collect::<Vec<_>>();

        // then ensure captured results contain exactly one uninitialized state
        assert_eq!(results.iter().filter(|v| !**v).count(), 1)
    }
}
