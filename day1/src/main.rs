//ownership
fn main() {
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{}",s);

    //move example
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{},{}",s1,s2) //s1 can't use, because has been freed
}
