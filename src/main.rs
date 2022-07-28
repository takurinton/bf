mod bf;
mod tests;

fn main() { 
    let code = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.";
    let result = bf::run(code.to_string());
    println!("{:?}", result);
}