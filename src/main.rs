mod bf;
mod tests;

fn main() { 
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
    println!("{:?}", output);
}