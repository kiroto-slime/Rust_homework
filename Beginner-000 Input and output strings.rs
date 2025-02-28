use std::io;
fn main() {
    let mut x:String=String::new();
    let _=io::stdin().read_line(&mut x);
    println!("[{}]", x.trim());
}
