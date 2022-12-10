#[allow(unused)]
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    #[should_panic]
    fn panic_on_index_out_of_bounds() {
        let list = [2, 2, 3];
        list.get(99).expect("Out of Bounds!");
    }

    // Using Result in tests
    #[test]
    fn result_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
