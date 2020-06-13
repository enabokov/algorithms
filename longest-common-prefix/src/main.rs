fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut common_prefix = String::new();

    if strs.is_empty() {
        return common_prefix;
    }

    let mut index_letter = 0;

    let mut add_letter = false;
    let mut letter = 'a';
    loop {
        let mut res = 0;
        for word in strs.iter() {
            if word.len() == 0 {
                return common_prefix;
            }
            match word.chars().nth(index_letter) {
                Some(value) => {
                    res = res + value as u32;
                    if res % value as u32 == 0 {
                        println!(
                            "word: {} - letters are similar -- {}, index: {}",
                            word, value, index_letter
                        );
                        add_letter = true;
                        letter = value;
                    } else {
                        println!(
                            "word: {} - letters are different - {}, index: {}",
                            word, value, index_letter
                        );
                        return common_prefix;
                    }
                }
                _ => return common_prefix,
            }
        }
        println!();
        if add_letter {
            common_prefix.push(letter);
            add_letter = false
        }

        index_letter += 1;
    }
}

fn main() {
    println!(
        "{}",
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );
}

#[macro_use]
extern crate time_test;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_convert {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    time_test!();
                    let (input, output) = $value;
                    assert_eq!(output, longest_common_prefix(input));
                }
            )*
        };
    }

    test_convert! {
        check_1: (vec!["flower".to_string(), "flow".to_string(), "flight".to_string()], "fl"),
        check_2: (vec!["dog".to_string(), "racecar".to_string(), "car".to_string()], ""),
        check_3: (vec!["dog".to_string(), "dolly".to_string(), "doll".to_string()], "do"),
        check_4: (vec![], ""),
        check_5: (vec!["".to_string()], ""),
        check_6: (vec!["a".to_string()], "a"),
        check_7: (vec!["a".to_string(), "cba".to_string(), "bcb".to_string()], ""),
        check_8: (vec!["abcdadcddfkcsdflkmvsdlfkvxmlsfv".to_string(), "asdflmvskdjfxmvnemlxrkncqmrcasdjcasdcasdc".to_string(), "albcmlaksdbclbsdflkjasdx;ckmq;wxclasdc".to_string()], "a"),
        check_9: (vec!["abc".to_string(), "abc".to_string(), "abc".to_string()], "abc"),
    }
}
