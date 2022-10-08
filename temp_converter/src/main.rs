fn main() {
   let celsius_temp = 23.0;
   let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

   assert_eq!(fahrenheit_temp,73.4);
   println!("Test Passed!");
}

fn celsius_to_fahrenheit(x:f32)->f32{
   (9.0 * x  / 5.0 ) + 32.0
}