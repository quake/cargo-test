use lib2::foo;

fn main() {
    println!("foo {}", foo());
}

#[cfg(test)]
mod tests {
    use lib2::foo;

    #[test]
    fn eq() {
        assert_eq!(1, foo());
    }
}
