use std::collections::HashMap;

fn convert(x: String) -> i32 {
    let mut symbols: HashMap<String, i32> = HashMap::new();
    symbols.insert("I".to_string(), 1);
    symbols.insert("V".to_string(), 5);
    symbols.insert("X".to_string(), 10);
    symbols.insert("L".to_string(), 50);
    symbols.insert("C".to_string(), 100);
    symbols.insert("D".to_string(), 500);
    symbols.insert("M".to_string(), 1000);

    let mut res = 0;
    let mut prev_value = -1;
    for c in x.chars() {
        // println!("{}", c);
        match symbols.get(&c.to_string()) {
            Some(value) => {
                if prev_value == -1 {
                    prev_value = *value;
                    res += value;
                } else {
                    if prev_value >= *value {
                        res += value;
                    } else {
                        res = value - res;
                    }
                }
            },
            None => println!("{} was not found", x)
        }
    }

    res
}

fn main() {
    println!("Result: {}", convert("IX".to_string()))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_convert_9() {
        assert_eq!(convert("IX".to_string()), 9);
    }

    #[test]
    fn test_check_convert_3() {
        assert_eq!(convert("III".to_string()), 3);
    }

    #[test]
    fn test_check_convert_6() {
        assert_eq!(convert("VI".to_string()), 6);
    }

    #[test]
    fn test_check_convert_4() {
        assert_eq!(convert("IV".to_string()), 4);
    }
}