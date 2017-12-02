use super::*;
use std::thread;
use galvanic_test::*;
use concurrency_primitives::SYSTEM_WIDE_SINGLETON_SENTINEL;

static GLOBAL_TEST_MUTEX: AtomicBool = AtomicBool::new(false);

test_suite! {
    name system_wide_singleton;
    use super::*;

    fixture system_wide_singleton() -> SystemWideSingleton {
        setup(&mut self) {
            // Ensure tests which manipulate app_state (which is backed by static state) never run simultaneously
            while GLOBAL_TEST_MUTEX.compare_and_swap(false, true, Ordering::Relaxed) {}

            SystemWideSingleton::new()
        }

        tear_down(&self) {
            // Reset 'static atomic backing store to uninstantiated to enable use in multiple tests
            SYSTEM_WIDE_SINGLETON_SENTINEL.compare_and_swap(true, false, Ordering::Relaxed);

            while !GLOBAL_TEST_MUTEX.compare_and_swap(true, false, Ordering::Relaxed) {}
        }
    }

    test first_call_reports_uninstantiated_single_threaded(system_wide_singleton()) {
        // given
        let subject = system_wide_singleton.val;

        // then when
        assert_eq!(subject.already_instantiated(), false);
    }

    test calls_after_first_report_instantiated_single_threaded(system_wide_singleton()) {
        // given
        let subject = system_wide_singleton.val;
        assert_eq!(subject.already_instantiated(), false);

        // then when
        assert_eq!(subject.already_instantiated(), true);
        assert_eq!(subject.already_instantiated(), true);
        assert_eq!(subject.already_instantiated(), true);
    }

    test many_calls_report_uninstantiated_exactly_once_multi_threaded(system_wide_singleton()) {
        // given
        const N_THREADS: usize = 8;
        let subject = system_wide_singleton.val;

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
