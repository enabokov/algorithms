fn compare(triangle1: Vec<Vec<i32>>, triangle2: Vec<Vec<i32>>) -> bool {
    let mut res = true;

    for it in triangle1.iter().zip(triangle2.iter()) {
        let (ait, bit) = it;
        if ait != bit {
            res = false;
        }
    }

    res
}

fn generate_new_level(l: &Vec<i32>) -> Vec<i32> {
    println!("process {:?}", l);
    if l.len() == 1 {
        return vec![1, 1];
    }

    let mut index = 0;
    let mut res = vec![];
    let mut left = 0;
    let mut right = l.get(index).unwrap();
    for _ in 0..l.len() + 1 {
        println!("left: {}, right: {}, res: {}", left, right, left + right);
        res.push(left + right);
        left = *right;
        index += 1;
        match l.get(index) {
            Some(right_value) => right = right_value,
            None => {
                println!("reached right bound, move left bound");
                left = *right;
                right = &0;
            }
        }
    }

    res
}

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::default();

    let root = vec![1];
    res.push(root);

    for _ in 0..num_rows - 1 {
        let new_level = generate_new_level(&res.last().unwrap());
        res.push(new_level);
    }

    res
}

fn main() {
    println!("{:?}", generate(11));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_convert {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (num, expected) = $value;
                    let triangle = generate(num);
                    assert!(compare(triangle, expected), true);
                }
            )*
        };
    }

    test_convert! {
        check_1: (3, vec![vec![1], vec![1, 1], vec![1, 2, 1]]),
    }
}
