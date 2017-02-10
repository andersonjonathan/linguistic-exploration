pub fn is_equal(a: String, b: String) -> bool {
    // compare strings
    let mut res = false;
    if a.len() == b.len() {
        for i in 0..a.len()-1 {
            if a.as_bytes()[i] == b.as_bytes()[i] {
                res = true;
            } else {
                res = false;
                break;
            }
        }
    }
   return res;
}

pub fn is_palindrome(s: String) -> bool {
    let mut original: String = s.clone();
    original = original.to_lowercase();
    let mut old_str: String = String::new();
    let mut new_str: String = String::new();
    for _ in 0..original.len() {
        let c = original.pop().unwrap();
        if c > ' ' {
            new_str.push(c);
            old_str.insert(0, c);
        }
    }
    return is_equal(old_str, new_str);
}
