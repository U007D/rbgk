use super::*;
use std::thread;
use galvanic_test::*;
#[allow(unused_imports)] use galvanic_assert::matchers::*;
use concurrency_primitives::APP_ALREADY_INSTANTIATED;

static GLOBAL_TEST_MUTEX: AtomicBool = AtomicBool::new(false);

test_suite! {
    name app_state;
    use super::*;

    fixture app_state() -> AppState {
        setup(&mut self) {
            // Ensure tests which manipulate app_state (which is backed by static state) never run simultaneously
            while GLOBAL_TEST_MUTEX.compare_and_swap(false, true, Ordering::Relaxed) {}

            AppState::new()
        }

        tear_down(&self) {
            // Reset 'static atomic backing store to uninstantiated to enable use in multiple tests
            APP_ALREADY_INSTANTIATED.compare_and_swap(true, false, Ordering::Relaxed);

            while !GLOBAL_TEST_MUTEX.compare_and_swap(true, false, Ordering::Relaxed) {}
        }
    }

    test first_call_reports_uninstantiated_single_threaded(app_state()) {
        // given
        let subject = app_state.val;

        // then when
        assert_eq!(subject.already_instantiated(), false);
    }

    test calls_after_first_report_instantiated_single_threaded(app_state()) {
        // given
        let subject = app_state.val;
        assert_eq!(subject.already_instantiated(), false);

        // then when
        assert_eq!(subject.already_instantiated(), true);
        assert_eq!(subject.already_instantiated(), true);
        assert_eq!(subject.already_instantiated(), true);
    }

    test many_calls_report_uninstantiated_exactly_once_multi_threaded(app_state()) {
        // given
        const N_THREADS: usize = 8;
        let subject = app_state.val;

        // when
        let results = (0..N_THREADS).map(|_| subject.clone())
                                    .map(|sut| thread::spawn(move || -> bool {
                                            sut.already_instantiated()
                                        }))
                                    .collect::<Vec<_>>()  // Spawn threads asynchronously
                                    .into_iter()
                                    .map(|h| h.join().unwrap())
                                    .collect::<Vec<_>>();

        // then ensure captured results contain exactly one uninstantiated state
        assert_eq!(results.iter().filter(|v| !**v).count(), 1);
    }
}
