# Kevlar Test Harness

A test harness for writing integration / regression tests in Rust.

**NOTE** This project is WIP and not ready for production use.

## Example usage

```rust
use Kevlar::prelude::*; // prelude not yet implemented

fn main() {
    let harness = TestHarness::new("my_test_name", PathBuf::from("./config.json"));
    let my_test = MyTest::default();
    harness.run(my_test);
}

#[derive(Default)]
struct MyTest {
    internal_data: String
}

impl TestCase for MyTest {
    fn run(&mut self, test_config: TestConfig, test_result: TestResult) -> TestResult {
        // Do something interesting here.
        
        test_result
    }
}
```

## TODO

- Selenium support
- SUT setup/teardown providers
- junit file support

and much more!
