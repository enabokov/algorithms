#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn create_list(nodes_values: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = Option::default();
    for val in nodes_values {
        let new_node = Box::new(ListNode::new(*val));
        if root.is_none() {
            root = Option::from(new_node);
        } else {
            insert_node(&mut root, &new_node);
        }
    }
    root
}

fn insert_node(l: &mut Option<Box<ListNode>>, data: &Box<ListNode>) {
    let mut tmp = l;
    while let Some(node) = tmp {
        tmp = &mut node.next;
    }

    tmp.get_or_insert(data.clone());
}

fn add(
    l1: &Option<Box<ListNode>>,
    l2: &Option<Box<ListNode>>,
    addup: i32,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => {
            if addup == 0 {
                return None;
            }
            Some(Box::new(ListNode::new(addup)))
        }
        (Some(node1), None) => Some(Box::new(ListNode {
            val: (node1.val + addup) % 10,
            next: add(
                &None,
                &node1.next,
                (node1.val + addup - (node1.val + addup) % 10) / 10,
            ),
        })),
        (None, Some(node2)) => Some(Box::new(ListNode {
            val: (node2.val + addup) % 10,
            next: add(
                &None,
                &node2.next,
                (node2.val + addup - (node2.val + addup) % 10) / 10,
            ),
        })),
        (Some(node1), Some(node2)) => Some(Box::new(ListNode {
            val: (node1.val + node2.val + addup) % 10,
            next: add(
                &node1.next,
                &node2.next,
                (node1.val + node2.val + addup - (node1.val + node2.val + addup) % 10) / 10,
            ),
        })),
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add(&l1, &l2, 0)
}

fn main() {
    let res = &create_list(&vec![0]);
    println!("Res: {:?}", res);
    // let res = covert_number_to_list(123);
    let res = add_two_numbers(
        create_list(&vec![9]),
        create_list(&vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
    );
    println!("Result: {:?}", res);
    let res = add_two_numbers(create_list(&vec![1, 2, 3]), create_list(&vec![3, 2, 1]));
    println!("Result: {:?}", res)
}

fn compare(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
    let mut res = true;

    let mut reflist1 = l1.as_ref();
    let mut reflist2 = l2.as_ref();
    loop {
        if reflist1.is_none() && reflist2.is_none() {
            break;
        }

        if reflist1.is_none() {
            res = false;
            break;
        }

        if reflist2.is_none() {
            res = false;
            break;
        }

        let node1 = reflist1.unwrap();
        let node2 = reflist2.unwrap();
        if node1.val != node2.val {
            println!("these are not equal: {} and {}", node1.val, node2.val);
            res = false;
            break;
        }

        reflist1 = node1.next.as_ref();
        reflist2 = node2.next.as_ref();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_convert {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (l1, l2, expected) = $value;
                    let actual_list1 = create_list(&l1);
                    let actual_list2 = create_list(&l2);
                    let actual_result = add_two_numbers(actual_list1, actual_list2);
                    let expected_list = create_list(&expected);
                    assert!(compare(actual_result, expected_list));
                }
            )*
        };
    }

    test_convert! {
        check_1: (vec![1, 2, 3], vec![3, 2, 1], vec![4, 4, 4]),
        check_2: (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
        check_3: (vec![0], vec![0], vec![0]),
        check_4: (vec![9], vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
    }
}
