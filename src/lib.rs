pub mod testcase;
pub mod testconfig;
pub mod testharness;
pub mod testresult;

// Convenience re-exports.
pub use testcase::{AsyncTestCase, TestCase};
pub use testconfig::{ConfigType, TestConfig};
pub use testharness::TestHarness;
pub use testresult::{TestResult, TestStatus};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
