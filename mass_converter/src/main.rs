fn main() {
   gram(10.0);
   gram(20.0);
   gram(30.0);
   gram(40.0);
   gram(50.0); 
   pound(10000.0);
   pound(20000.0);
   pound(30000.0);
   pound(40000.0);
   pound(50000.0);
}
fn gram(x: f32){
    let result = 453.6 * x;
    println!("The weight is {result} grams");
}
fn pound(x:f32){
    let result = 0.002 * x;
    println!("The weight is {result} pounds");
}