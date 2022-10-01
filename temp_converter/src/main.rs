fn main() {
   ftoc(-100);
   ctof(-100);
}
fn ftoc(f:i32){
   let result = (f-32)*5/9;
   println!("The temperature in Celsuis is {result} degree.");
}
fn ctof(c:i32){
    let result = (c*9/5)+32;
    println!("The temperature in Fahrenheit is {result} degree.");
}