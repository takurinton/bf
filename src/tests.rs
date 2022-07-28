#[cfg(test)]
mod lexer_tests {
    use crate::bf;

    #[test]
    fn lexer_length() {
        let code = "><+-.,[]";
        let tokens = bf::lexer(code.to_string());
        assert_eq!(tokens.len(), 8);
    }

    #[test]
    fn lexer() {
        let code = "><+-.,[]";
        let tokens = bf::lexer(code.to_string());
        assert_eq!(tokens[0], bf::Token::MoveRight);
        assert_eq!(tokens[1], bf::Token::MoveLeft);
        assert_eq!(tokens[2], bf::Token::Increment);
        assert_eq!(tokens[3], bf::Token::Decrement);
        assert_eq!(tokens[4], bf::Token::Output);
        assert_eq!(tokens[5], bf::Token::Input);
        assert_eq!(tokens[6], bf::Token::JumpForward);
        assert_eq!(tokens[7], bf::Token::JumpBackward);
    }
}