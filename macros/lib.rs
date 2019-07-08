#[macro_export]
macro_rules! foo {
    () => {
        pub fn f(input: &HashMap<u32, u32>) {}
    }
}
