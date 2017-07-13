fn main() {
    let answer = square(4);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
