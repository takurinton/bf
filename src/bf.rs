#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward,
    JumpBackward
}

impl Token {
    fn input_character(character: char) -> Option<Self> {
         match character {
            // https://ja.wikipedia.org/wiki/Brainfuck#Brainfuck%E3%81%AE%E8%A8%80%E8%AA%9E%E4%BB%95%E6%A7%98
            '>' => Some(Token::MoveRight),    // Move the pointer to the right.
            '<' => Some(Token::MoveLeft),     // Move the pointer to the left.
            '+' => Some(Token::Increment),    // Increment the value at the pointer.
            '-' => Some(Token::Decrement),    // Decrement the value at the pointer.
            '.' => Some(Token::Output),       // Output the value at the pointer.
            ',' => Some(Token::Input),        // Input a value at the pointer.
            '[' => Some(Token::JumpForward),  // Jump forward to the matching ] if the value at the pointer is zero.
            ']' => Some(Token::JumpBackward), // Jump backward to the matching [ if the value at the pointer is nonzero.
            _ => None                         // Ignore other characters.
        }
    }
}

pub fn lexer(code: String) -> Vec<Token> {
    code.chars().map(|c| Token::input_character(c)).filter(|c| c.is_some()).map(|c| c.unwrap()).collect()
}