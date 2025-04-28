#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }
    #[test]
    #[ignore]
    //#[should_panic(expected = "This test will fail")]
    fn it_fails() {
        panic!("This test will fail");
    }
    fn call_simple_add(){
        assert!(simple_add());
    }  
}
fn simple_add() -> bool {
    if 2+2 == 4 {
        true
    } else {
        false
    }
}