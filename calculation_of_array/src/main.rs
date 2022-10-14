fn main() {
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,-20,3];
    let mut max: i32 ;
    let mut min: i32 ;
    let mut mean :f64;
    max = numbers[0];
    min = numbers[0];
    mean = 0.0;
    for i in 0..numbers.len(){
         if numbers[i] > max{
            max = numbers[i];
         }else if numbers[i] < min {
            min = numbers[i];
         }
         mean += numbers[i] as f64;
    }
    mean /= numbers.len() as f64;
    println!("mean: {mean}");
    println!("max: {max}");
    println!("min: {min}");


}


