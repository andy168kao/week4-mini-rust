use std::io;

fn main() {
    let mut first_string = String::new();
    let mut second_string = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut first_string).unwrap();

    println!("Enter the second number: ");
    io::stdin().read_line(&mut second_string).unwrap();

    let first_num = first_string.trim().parse::<i32>().unwrap();
    let second_num = second_string.trim().parse::<i32>().unwrap();

    let sum = first_num + second_num;

    println!("The sum of {} and {} is: {}", first_num, second_num, sum);
}
