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
        TestCase::new("app::run()", "succeeds", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN an app
            let mock_width = 0;
            let expected_result = Ok(());
            let sut = App::new(Vec::new(), mock_width);

            // WHEN the app greets
            let result = sut.run();

            // THEN the result should contain the expected architecture width
            match result == expected_result {
                true => TestCaseStatus::PASSED,
                false => TestCaseStatus::FAILED,
            }
        })),
        TestCase::new("app::greet()", "returns expected message", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN an app
            let mock_width = 42;
            let expected_result = format!("Hello, {}-bit world!", mock_width);
            let sut = App::new(Vec::new(), mock_width);

            // WHEN the app greets
            let result = sut.greet();

            // THEN the result should contain the expected architecture width
            match result == expected_result {
                true => TestCaseStatus::PASSED,
                false => TestCaseStatus::FAILED,
            }
        })),
    ]);
}
