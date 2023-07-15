#[cfg(test)]
mod tests {
    use assert_cli::Assert;

    #[test]
    fn test_application() {
        let args = vec!["https://sachiniyer.com"];
        Assert::main_binary()
            .with_args(&args)
            .stdout()
            .is("https://sachiniyer.com: 200 OK")
            .unwrap()
    }

    #[test]
    fn test_stdin() {
        Assert::main_binary()
            .stdin("https://sachiniyer.com")
            .stdout()
            .is("https://sachiniyer.com: 200 OK")
            .unwrap()
    }

    #[test]
    fn test_file() {
        Assert::main_binary()
            .with_args(&["-i", "tests/test.txt"])
            .stdout()
            .is("https://sachiniyer.com: 200 OK\nhttps://sachiniyer.com: 200 OK")
            .unwrap()
    }

    #[test]
    fn test_redirect() {
        let args = vec!["https://ai.sachiniyer.com"];
        Assert::main_binary()
            .with_args(&args)
            .stdout()
            .is("https://ai.sachiniyer.com: 200 OK")
            .unwrap()
    }
}
