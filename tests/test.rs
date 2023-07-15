#[cfg(test)]
mod tests {
    use assert_cli::Assert;

    #[test]
    fn test_application() {
        let args = vec!["https://yatin.cc"];
        Assert::main_binary()
            .with_args(&args)
            .stdout()
            .is("https://yatin.cc: 200 OK")
            .unwrap()
    }

    #[test]
    fn test_stdin() {
        Assert::main_binary()
            .stdin("https://yatin.cc")
            .stdout()
            .is("https://yatin.cc: 200 OK")
            .unwrap()
    }

    #[test]
    fn test_file() {
        Assert::main_binary()
            .with_args(&["-i", "tests/test.txt"])
            .stdout()
            .is("https://yatin.cc: 200 OK\nhttps://yatin.cc: 200 OK")
            .unwrap()
    }
}
