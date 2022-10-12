fn main() {
    let mut count =0;
    let fruits = ["apple", "orange", "peach"];

    while count < fruits.len(){
        println!("Fruit is {}", fruits[count]);
        count += 1;
    }
}
