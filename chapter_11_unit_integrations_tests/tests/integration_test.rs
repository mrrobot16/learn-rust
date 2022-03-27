use chapter_11_unit_integrations_tests::{add_two};

// How to export functions inside the tests folder.
mod common;

#[cfg(test)]
mod tests {
    use super::{common};
    #[test]
    fn it_adds_two(){
        common::setup();
        let number = chapter_11_unit_integrations_tests::add_two(2);
        assert_eq!(4, number);
    }   
    
    #[test]
    fn it_adds_three() {
        let number = chapter_11_unit_integrations_tests::add_three(0);
        
        assert_eq!(3, number);
    } 
}