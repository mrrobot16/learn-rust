#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let equals = 2 + 2;
        assert_eq!(equals, 4);
    }
    
    #[test]
    #[should_panic]
    fn it_works2() {
        let equals = 2 + 3;
        assert_eq!(equals, 4);
    }
    
    #[test]
    #[ignore]
    fn do_not_run_me() {
        // println!();
        let equals = 2 + 3;
        assert_eq!(equals, 4, "Do not run me because i fail");
    }
}