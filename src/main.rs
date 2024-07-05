fn main() {
    println!("Hello, world!");
    let a = String::from("10");
    let b = a; // Use clone() to make a copy instead of moving ownership
    println!("{:?}", b);
    println!("{:?}", b);
    let c = test_fun();
    println!("{} ", c);
}

fn test_fun() -> i32 {
    4
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        assert_eq!(1, 1); // code for test1
    }
}
