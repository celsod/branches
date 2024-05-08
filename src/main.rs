fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_a = 6;

    if number_a % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_a % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_a % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_b = if condition {5} else {6};
    println!("The value of number is: {}", number_b);
}