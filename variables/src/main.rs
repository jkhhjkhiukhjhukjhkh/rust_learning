fn main() {
    let mut s1 = String::from("abcd");

    s1.push_str(" is not dcba!");
    let s2 = s1;


    println!("The value of s is: {}", s2);

}
