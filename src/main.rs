fn main() {
    println!("Hello, world!");
}

//回文数
fn is_palindrome(x: i32) -> bool {
    let mut tmp = x;
    if tmp < 0 || (tmp % 10 == 0 && tmp != 0) {
        return false;
    };
    let mut reverted_number = 0;
    while tmp > reverted_number {
        reverted_number = reverted_number * 10 + tmp % 10;
        tmp /= 10;
    };
    return tmp == reverted_number || tmp == reverted_number / 10;
}