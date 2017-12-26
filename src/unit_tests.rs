use std::cell::Cell;
use polish::test_case::{TestRunner, TestCaseStatus, TestCase, TEST_RUNNER_ATTRIBUTES as TestRunnerAttributes, TEST_RUNNER_TIME_UNITS as TestRunnerTimeUnits};
use polish::logger::Logger;
#[allow(unused_imports)] use super::*;

struct MockArch {
    width_times_called: Cell<usize>,
    width: usize,
}

impl MockArch {
    fn new(width: usize) -> Self { Self { width_times_called: Cell::new(0), width } }
}

impl Info for MockArch {
    fn width(&self) -> usize {
        self.width_times_called.set(self.width_times_called.get() + 1);
        self.width
    }
}

#[test]
fn tests() {
    TestRunner::new()
               .set_module_path(module_path!())
               .set_attributes(TestRunnerAttributes.disable_final_stats | TestRunnerAttributes.minimize_output)
               .set_time_unit(TestRunnerTimeUnits.microseconds)
               .run_tests(vec![
        TestCase::new("App::run()", "yields arch width", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN an app
            let mock_width = 42;
            let expected_result = Ok::<String, Error>(format!("Hello, {}-bit world!", mock_width));
            let mock = MockArch::new(mock_width);
            #[allow(result_unwrap_used)]
            let sut = App::new(&mock);

            // WHEN the app is run
            let result = sut.run();

            // THEN the result should contain the expected architecture width
            match result == expected_result {
                true => TestCaseStatus::PASSED,
                false => TestCaseStatus::FAILED,
            }
        })),
        TestCase::new("App::run()", "calls Info::width() once", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN an app
            let mock_width = 42;
            let mock = MockArch::new(mock_width);
            #[allow(result_unwrap_used)]
            let sut = App::new(&mock);

            // WHEN the app is run
            let _ = sut.run();

            // THEN the app should have called Info::width() exactly once
            match mock.width_times_called.get() == 1 {
                true => TestCaseStatus::PASSED,
                false => TestCaseStatus::FAILED,
            }
        })),
    ]);
}
