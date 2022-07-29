#[cfg(test)]
mod lexer_tests {
    use crate::bf;

    #[test]
    fn test_lexer_length() {
        let code = "><+-.,[]";
        let tokens = bf::lexer(code.to_string()).unwrap();
        assert_eq!(tokens.len(), 8);
    }

    #[test]
    fn test_lexer() {
        let code = "><+-.,[]";
        let tokens = bf::lexer(code.to_string()).unwrap();
        assert_eq!(tokens[0], bf::Token::MoveRight);
        assert_eq!(tokens[1], bf::Token::MoveLeft);
        assert_eq!(tokens[2], bf::Token::Increment);
        assert_eq!(tokens[3], bf::Token::Decrement);
        assert_eq!(tokens[4], bf::Token::Output);
        assert_eq!(tokens[5], bf::Token::Input);
        assert_eq!(tokens[6], bf::Token::JumpForward);
        assert_eq!(tokens[7], bf::Token::JumpBackward);
    }

    #[test]
    fn test_lexer_unexpected_token() {
        // code containts an unexpected token
        let code = "abc";
        let tokens = bf::lexer(code.to_string());
        assert_eq!(tokens, Err(bf::Error::UnknownTokenError));
    }
}

#[cfg(test)]
mod run_tests {
    use crate::bf;

    #[test]
    fn run_test_hello_world() {
        // "Hello World!\n", code includes "/n"
        let code = "++++++++++
        [
            >+++++++
            >++++++++++
            >+++++++++++
            >+++
            >+++++++++
            >+
            <<<<<<-
        ]
        >++.
        >+.
        >--..
        +++.
        >++.
        >---.
        <<.
        +++.
        ------.
        <-.
        >>+.
        >>.";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output.unwrap(), "Hello World!\n");
    }

    #[test]
    fn run_test_takurinton() {
        // "takurinton\n", code execludes "/n"
        let code = "+++++++++[>++++++++++++<-]>++++++++.<+++++++++[>--<-]>-.<+++++++++[>+<-]>+.<+++++++++[>+<-]>+.---.<+++++++++[>-<-]>.+++++.++++++.-----.-.";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output.unwrap(), "takurinton");
    }

    #[test]
    fn test_run_move_right_overflow() {
        // [...Array(257)].map(_ => ">").join("")
        // overflow maximum pointer
        let code = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output, Err(bf::Error::RuntimeError));
    }

    #[test]
    fn test_run_move_left_underflow() {
        // underflow inumum pointer
        let code = "<<";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output, Err(bf::Error::RuntimeError));
    }

    #[test]
    fn test_run_increment_memory_overflow() {
        let code = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output, Err(bf::Error::RuntimeError));
    }

    #[test]
    fn test_run_decrement_memory_overflow() {
        let code = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>-";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output, Err(bf::Error::RuntimeError));
    }

    #[test]
    fn test_run_not_close_jump_forward() {
        // jump forward without close token when pointer is 0
        let code = "[[[[";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output, Err(bf::Error::RuntimeError));
    }

    #[test]
    fn test_run_not_open_jump_backward() {
        // jump backward without open token when pointer is not 0
        let code = "+++]]]]";
        let tokens = bf::lexer(code.to_string());
        let output = bf::run(tokens);
        assert_eq!(output, Err(bf::Error::RuntimeError));
    }
}
