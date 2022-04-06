//ownership
fn main() {
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{}",s);

    //move example
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{},{}",s1,s2); //s1 can't use, because has been freed

    //copy example, stored in stack
    let x = 5;
    let y = x;
    println!("{},{}", x, y);

    //clone example, 
    let s3: String = String::from("hello");
    let s4 = s3.clone();
    println!("{},{}", s3, s4);
}
