import unittest
from palindrome import is_palindrome


class TestPalindrome(unittest.TestCase):
    def test_is_palindrome(self):
        self.assertTrue(is_palindrome("Was it a cat I saw"))
        self.assertTrue(is_palindrome("No x in Nixon"))

    def test_is_not_palindrome(self):
        self.assertFalse(is_palindrome("asdasd"))

if __name__ == '__main__':
    unittest.main()
