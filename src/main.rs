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
    let result = match output {
        Ok(output) => output,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };
    println!("{}", result);
}
