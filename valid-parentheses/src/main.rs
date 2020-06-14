fn is_valid(s: String) -> bool {
    let mut res = true;

    let mut opened_brackets = Vec::<char>::default();
    for c in s.trim().chars() {
        if c == '{' || c == '(' || c == '[' {
            println!("Push {}", c);
            opened_brackets.push(c);
        } else {
            if let Some(opened_bracket) = opened_brackets.pop() {
                let expected_closed_bracket = match opened_bracket {
                    '{' => '}',
                    '[' => ']',
                    '(' => ')',
                    _ => {
                        println!("Bad bracket: {}", opened_bracket);
                        break;
                    }
                };

                if expected_closed_bracket != c {
                    res = false;
                    break;
                }
            } else {
                res = false;
                break;
            }
        }
    }

    if !opened_brackets.is_empty() {
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
                    let (brackets, expected) = $value;
                    assert_eq!(expected, is_valid(brackets.to_string()));
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
