fn main() {
    let x = fib(10);
    println!("the 10th fibonacci number is {x}");
    let y = fib(11);
    println!("the 11th fibonacci number is {y}");
    let z = fib(12);
    println!("the 12th fibonacci number is {z}");
}

fn fib(x: i32) -> i32 {
    if x <= 1 {
        x
    }else{
        fib(x-1) + fib(x-2)
    }
}