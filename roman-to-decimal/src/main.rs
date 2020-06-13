fn convert(s: String) -> i32 {
    let mut res = 0;
    let mut prev_value = -1;
    for c in s.chars().rev() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => -1,
        };
        if prev_value == -1 {
            res += value;
        } else {
            if prev_value > value {
                res = res - value;
            } else {
                res = res + value;
            }
        }
        prev_value = value;
    }

    res
}

fn main() {
    println!("Result: {}", convert("CLIV".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_convert {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (roman, decimal) = $value;
                    assert_eq!(decimal, convert(roman.to_string()));
                }
            )*
        };
    }

    test_convert! {
        check_1: ("I", 1),
        check_3: ("III", 3),
        check_4: ("IV", 4),
        check_6: ("VI", 6),
        check_9: ("IX", 9),
        check_57: ("LVII", 57),
        check_89: ("LXXXIX", 89),
        check_99: ("XCIX", 99),
        check_154: ("CLIV", 154),
        check_1994: ("MCMXCIV", 1994),
    }
}
