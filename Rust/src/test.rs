#[cfg(test)]
mod test {
	use ::code::*;
	#[test]
	fn test_is_equal() {
		assert_eq!(is_equal(String::from("test"), String::from("test")), true);
        assert_eq!(is_equal(String::from("asd"), String::from("asd")), true);
	}
    #[test]
    #[should_panic]
	fn test_is_not_equal() {
		assert_eq!(is_equal(String::from("tes t"), String::from("test")), true);
	}
    #[test]
	fn test_is_palindrome() {
		assert_eq!(is_palindrome(String::from("Was it a cat I saw")), true);
        assert_eq!(is_palindrome(String::from("No x in Nixon")), true);
	}
    #[test]
    #[should_panic]
	fn test_is_not_palindrome() {
		assert_eq!(is_palindrome(String::from("asdas")), true);
	}

}

