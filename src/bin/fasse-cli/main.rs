pub fn main() {
    println!("fasse ver : {}", fasse::VERSION);
}

#[cfg(test)]
mod test {
    use super::main;
    #[test]
    fn test_main() {
        main();
    }
}
