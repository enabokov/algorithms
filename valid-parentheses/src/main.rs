fn is_valid(s: String) -> bool {
    let mut res = true;

    let mut opened_parentheses = Vec::<char>::default();
    for c in s.trim().chars() {
        if c == '{' || c == '(' || c == '[' {
            println!("Push {}", c);
            opened_parentheses.push(c);
        } else {
            if let Some(prev) = opened_parentheses.pop() {
                if prev == '{' && c != '}' {
                    res = false;
                    break;
                }
                if prev == '[' && c != ']' {
                    res = false;
                    break;
                }
                if prev == '(' && c != ')' {
                    res = false;
                    break;
                }
            } else {
                res = false;
                break;
            }
        }
    }

    if !opened_parentheses.is_empty() {
        res = false;
    }

    res
}

fn main() {
    println!("{}", is_valid("{}[]".to_string()));
    println!("{}", is_valid("{[]}".to_string()));
    println!("{}", is_valid("{[}".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_convert {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (parentheses, expected) = $value;
                    assert_eq!(expected, is_valid(parentheses.to_string()));
                }
            )*
        };
    }

    test_convert! {
        check_1: ("", true),
        check_2: ("{}", true),
        check_3: ("{}[]", true),
        check_4: (" ", true),
        check_5: ("[{}]]", false),
        check_6: ("({})", true),
        check_7: ("[", false),
    }
}
