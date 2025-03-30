use std::io::stdin;
fn main() {
    let mut y:String= String::new();
    let _= stdin().read_line(&mut y);
    let a= y.trim().parse::<f32>().unwrap();
    let pi= 3.14;
    if a<0.0{println!("Wrong");}
    else{println!("{} {}",a.powi(2)*pi, 2.0*pi*a);}
}
