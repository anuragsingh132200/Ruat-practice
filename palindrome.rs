pub fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}
