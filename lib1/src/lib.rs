#[cfg(not(feature = "feature1"))]
pub fn bar() -> u8 {
    0
}

#[cfg(feature = "feature1")]
pub fn bar() -> u8 {
    1
}
