pub fn run() {
    println!("str and string run !");

    let stack_a_str = "this str is fixed on stack!";

    let string_at_heap = String::from("this string is fixed on heap!");

    let my_data_from_a_string = "123321123321";
    let result = pass_one_character(&my_data_from_a_string).unwrap();
    println!("result: {:?}", result);
}

fn pass_one_character(str: &str) -> Result<Vec<MyData>, ParseError> {
    println!("str: {}", str);
    if str.len() % 3 != 0 {
        return Err(ParseError::NotRightLength);
    }
    let mut chars = str.chars();
    let mut all = Vec::new();
    while let Some(c) = chars.next() {
        if let Some(c) = c.to_digit(10) {
            let mut data = [c as i32; 3];
            for i in 1..3 {
                let c = match chars.next() {
                    Some(c) => c,
                    None => return Err(ParseError::NotRightLength),
                };
                data[i] = c.to_digit(10).ok_or(ParseError::NotNumber)? as i32;
            }
            all.push(MyData(data));
        } else {
            return Err(ParseError::NotNumber);
        }
    }
    Ok(all)
}

#[derive(Debug)]
struct MyData([i32; 3]);
#[derive(Debug)]
enum ParseError {
    NotRightLength,
    NotNumber
}
