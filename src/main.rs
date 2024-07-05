fn main() {
    println!("Hello, world!");
    let a = String::from("10");
    let b = a.clone(); // Use clone() to make a copy instead of moving ownership
    println!("{:?}", b);
    println!("{:?}", b);
    // println!("{}", a); // Uncommenting this line will cause a compile error due to ownership transfer

    let c = test_fun();
    println!("{} ", c);
}

fn test_fun() -> i32 {
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(1, 1);
    }
}
