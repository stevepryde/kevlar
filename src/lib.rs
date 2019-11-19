pub mod testcase;
pub mod testconfig;
pub mod testharness;
pub mod testresult;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
