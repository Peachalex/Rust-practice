fn main() {
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,-20,3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut sum :i32 = 0;
    for i in 0..numbers.len(){
         sum += numbers[i];
         if numbers[i] > max{
            max = numbers[i];
         }else if numbers[i] < min {
            min = numbers[i];
         }
    }
    let mean : i64 = (sum as usize / numbers.len()).try_into().unwrap();
    println!("sum: {sum}");
    println!("mean: {mean}");
    println!("max: {max}");
    println!("min: {min}");


}


