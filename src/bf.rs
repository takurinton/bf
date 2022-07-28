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
    fn from_str(character: char) -> Option<Self> {
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
    code.chars().map(|c| Token::from_str(c)).filter(|c| c.is_some()).map(|c| c.unwrap()).collect()
}

pub fn run(code: String) -> String {
    let tokens = lexer(code);
    let mut memory = vec![0; 30000];
    let mut pointer = 0;
    let mut input = vec![];
    let mut output = vec![];
    for token in tokens {
        match token {
            Token::MoveRight => pointer += 1,
            Token::MoveLeft => pointer -= 1,
            Token::Increment => memory[pointer] += 1,
            Token::Decrement => memory[pointer] -= 1,
            Token::Output => {
                let value = memory[pointer] as u8 as char;
                output.push(value);
            }
            Token::Input => {
                let input_value = input.pop().unwrap();
                memory[pointer] = input_value;
            }
            Token::JumpForward => {
                // TODO
            }
            Token::JumpBackward => {
                // TODO
            }
        }
    }

    output.iter().map(|c| c.to_string()).collect()
}