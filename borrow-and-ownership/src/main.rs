fn main() {
    let mut str1 = String::from("Hello");
    str1.push_str(" from Rust!!");
    println!("{}", str1);
    // It errors out for the bottom line:
    // let str2 = str1;
    // But I can borrow the value, as the borrowed ref does not outlive the original value.
    let _str2 = &mut str1;
    println!("{}", str1);
}
