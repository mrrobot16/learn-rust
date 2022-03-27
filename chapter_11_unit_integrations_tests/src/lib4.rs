#[cfg(test)]
mod tests {
    #[test]
    fn it_returns_result() -> Result<(), String> {
        let equals = 2 + 2 == 4;
        if equals {
            Ok(())
        }
        else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}