fn main() {
    println!("Hello, world!");
    let a = String::from("10");
    let b = a;
    println!("{:?}", b);
    println!("{:?}", b);
    // println!("{}", a);
    let c = test_fun();
    println!("{} ", c);
}

fn test_fun() -> i32 {
    4
}

// fn test2() {
//     println!("fdstat");
// }

#[test]
fn test_main() {
    assert_eq!(1, 1);
}
