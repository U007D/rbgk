use polish::test_case::{TestRunner, TestCaseStatus, TestCase, TEST_RUNNER_ATTRIBUTES as TestRunnerAttributes, TEST_RUNNER_TIME_UNITS as TestRunnerTimeUnits};
use polish::logger::Logger;
#[allow(unused_imports)] use super::*;

#[test]
fn tests() {
    TestRunner::new()
               .set_module_path(module_path!())
               .set_attributes(TestRunnerAttributes.disable_final_stats | TestRunnerAttributes.minimize_output)
               .set_time_unit(TestRunnerTimeUnits.microseconds)
               .run_tests(vec![
        TestCase::new("app::run()", "returns expected message", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN an app
            let mock_width = 42;
            let expected_result = Ok::<String, Error>(format!("Hello, {}-bit world!", mock_width));
            #[allow(result_unwrap_used)]
            let sut = App::new(Vec::new(), mock_width);

            // WHEN the app is run
            let result = sut.run();

            // THEN the result should contain the expected architecture width
            match result == expected_result {
                true => TestCaseStatus::PASSED,
                false => TestCaseStatus::FAILED,
            }
        })),
    ]);
}
