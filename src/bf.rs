#[derive(Debug, PartialEq)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward,
    JumpBackward,
}

impl Token {
    fn from_str(character: char) -> Option<Self> {
        match character {
            // https://ja.wikipedia.org/wiki/Brainfuck#Brainfuck%E3%81%AE%E8%A8%80%E8%AA%9E%E4%BB%95%E6%A7%98
            '>' => Some(Token::MoveRight), // Move the pointer to the right.
            '<' => Some(Token::MoveLeft),  // Move the pointer to the left.
            '+' => Some(Token::Increment), // Increment the value at the pointer.
            '-' => Some(Token::Decrement), // Decrement the value at the pointer.
            '.' => Some(Token::Output),    // Output the value at the pointer.
            ',' => Some(Token::Input),     // Input a value at the pointer.
            '[' => Some(Token::JumpForward), // Jump forward to the matching ] if the value at the pointer is zero.
            ']' => Some(Token::JumpBackward), // Jump backward to the matching [ if the value at the pointer is nonzero.
            _ => None,                        // Ignore other characters.
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    RuntimeError,
    UnknownTokenError,
}

pub fn lexer(code: String) -> Result<Vec<Token>, Error> {
    let tokens = code
        .trim()
        .chars()
        .filter(|c| match c {
            ' ' | '\n' | '\r' | '\t' => false,
            _ => true,
        })
        .map(|character| {
            Ok(match Token::from_str(character) {
                Some(token) => token,
                None => return Err(Error::UnknownTokenError),
            })
        })
        .collect::<Result<Vec<Token>, Error>>()?;
    Ok(tokens)
}

pub fn run(tokens: Result<Vec<Token>, Error>) -> Result<String, Error> {
    let mut memory = vec![0; 256]; // 30000 isn't enough for me.
    let mut pointer = 0;
    let mut input = vec![];
    let mut output = String::new();
    let mut index = 0;
    let tokens = tokens.unwrap();
    while index < tokens.len() {
        match tokens[index] {
            Token::MoveRight => match pointer + 1 {
                256 => return Err(Error::RuntimeError),
                _ => pointer += 1,
            },
            Token::MoveLeft => match pointer + 1 {
                0 => return Err(Error::RuntimeError),
                _ => pointer -= 1,
            },
            Token::Increment => match pointer + 1 {
                256 => return Err(Error::RuntimeError),
                _ => memory[pointer] += 1,
            },
            Token::Decrement => match pointer + 1 {
                0 => return Err(Error::RuntimeError),
                _ => memory[pointer] -= 1,
            },
            Token::Output => {
                let value = memory[pointer] as u8 as char;
                match value {
                    // TODO: 何があったらエラー？
                    _ => output.push(value),
                }
            }
            Token::Input => {
                let input_value = input.pop().unwrap();
                match input_value {
                    _ => memory[pointer] = input_value,
                }
            }
            Token::JumpForward => match memory[pointer] {
                0 => {
                    let mut depth = 1;
                    while depth > 0 {
                        match index + 1 {
                            256 => return Err(Error::RuntimeError),
                            _ => match tokens[index + 1] {
                                Token::JumpForward => depth += 1,
                                Token::JumpBackward => depth -= 1,
                                _ => (),
                            },
                        }
                        index += 1;
                    }
                }
                _ => {}
            },
            Token::JumpBackward => match memory[pointer] {
                0 => {}
                _ => {
                    let mut depth = 1;
                    while depth > 0 {
                        match index - 1 {
                            0 => return Err(Error::RuntimeError),
                            _ => match tokens[index - 1] {
                                Token::JumpForward => depth -= 1,
                                Token::JumpBackward => depth += 1,
                                _ => (),
                            },
                        }
                        index -= 1;
                    }
                }
            },
        }
        index += 1;
    }

    Ok(output)
}
