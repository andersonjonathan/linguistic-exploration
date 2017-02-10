def is_palindrome(palindrome):
    original = palindrome.lower().replace(' ', '')
    reverse = []
    for c in original:
        reverse.insert(0, c)
    return original == "".join(reverse)
