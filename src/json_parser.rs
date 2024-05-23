// write step 作为新手实践的步骤

pub fn run() {
    println!("JSON parser run!");
}

pub enum JsonValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}
enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEndOfInput,
    InvalidNumber, // 其他可能的错误类型...
}

pub fn parse(input: &str) -> Result<(JsonValue, &str), ParseError> {
    // TODO: 实现解析逻辑
    // 递归下降解析 !!!
    let input = input.trim_start();
    match input.chars().next() {
        Some('"') => parse_string(input),
        Some('[') => parse_array(input),
        Some('{') => parse_object(input),
        Some('t') | Some('f') => parse_boolean(input),
        Some('n') => parse_null(input),
        Some(c) if c.is_digit(10) || c == '-' => parse_number(input),
        _ => Err(ParseError::UnexpectedToken),
    }
}

fn parse_string(input: &str) -> Result<(JsonValue, &str), ParseError> {
    if !input.starts_with('\"') {
        return Err(ParseError::UnexpectedToken);
    }

    let mut input = &input[1..];
    let mut string = String::new();

    loop {
        let (idx, c) = input.char_indices().next().unwrap();
        match c {
            '\"' => {
                input = &input[idx+1..];
                break;
            },
            c => {
                string.push(c);
                input = &input[idx+1..];
            },
        }
    }

    Ok((JsonValue::String(string), input))
}

fn parse_null(input: &str) -> Result<(JsonValue, &str), ParseError> {
    // TODO: 实现解析 null 的逻辑
    // write step 1
    // 分析: 解析知道是null
    // json  ?:null
    // 解析的函数这里，只关注传入的这个值
    match input.strip_prefix("null") {
        Some(rest) => Ok((JsonValue::Null, rest)),
        None => Err(ParseError::UnexpectedToken),
    }
}

fn parse_boolean(input: &str) -> Result<(JsonValue, &str), ParseError> {
    // TODO: 实现解析 boolean 的逻辑
    if let Some(rest) = input.strip_prefix("true") {
        return Ok((JsonValue::Bool(true), rest));
    } else if let Some(rest) = input.strip_prefix("false") {
        return Ok((JsonValue::Bool(false), rest));
    } else {
        return Err(ParseError::UnexpectedToken);
    }
}

fn parse_number(input: &str) -> Result<(JsonValue, &str), ParseError> {
    // TODO: 实现解析 number 的逻辑
    // : number ,  float , int

    // 考究值的有效性
    let end = input
        .chars()
        // 第一个是把最后一个字符看看是不是数字
        // 然后还有关于是不是小数点结尾的判断
        .take_while(|c| c.is_digit(10) || *c == '.')
        .count();
    let (number_str, remaining) = input.split_at(end);
    if number_str.contains('.') {
        match number_str.parse::<f64>() {
            Ok(number) => Ok((JsonValue::Number(Number::Float(number)), remaining)),
            Err(_) => Err(ParseError::InvalidNumber),
        }
    } else {
        match number_str.parse::<i64>() {
            Ok(number) => Ok((JsonValue::Number(Number::Int(number)), remaining)),
            Err(_) => Err(ParseError::InvalidNumber),
        }
    }
}
fn parse_array(input: &str) -> Result<(JsonValue, &str), ParseError> {
    if !input.starts_with('[') {
        return Err(ParseError::UnexpectedToken);
    }

    let mut input = &input[1..];
    let mut elements = Vec::new();

    loop {
        input = input.trim_start();

        if input.starts_with(']') {
            return Ok((JsonValue::Array(elements), &input[1..]));
        }

        let (element, remaining_input) = parse(input)?;
        elements.push(element);

        input = remaining_input.trim_start();
        if input.starts_with(',') {
            input = &input[1..];
        } else if !input.starts_with(']') {
            return Err(ParseError::UnexpectedToken);
        }
    }
}

fn parse_object(input: &str) -> Result<(JsonValue, &str), ParseError> {
    // TODO: 实现解析 object 的逻辑
    unimplemented!();
}
