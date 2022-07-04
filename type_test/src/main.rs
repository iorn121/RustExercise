fn main() {
    fn func(code: i32) -> Result<i32, String> {
        println!("{}", code);
        Ok(100)
    }
    let some_value: Result<i32, String>=Ok(200);
    // println!("code: {}",some_value.unwrap_or(-1));
    let next_result=some_value.and_then(func);
    }