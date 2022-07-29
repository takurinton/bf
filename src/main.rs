mod bf;
mod tests;

fn main() {
    // print "Hello World!\n"
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
    match output {
        Ok(output) => println!("{}", output),
        Err(error) => println!("{:?}", error),
    }
}
