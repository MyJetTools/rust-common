pub fn is_decorator_symbol(c: char) -> bool {
    c == '*' || c == '_' || c == '~'
}
