use lib1::bar;

pub fn foo() -> u8 {
    bar()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib2_foo() {
        assert_eq!(1, foo());
    }
}
