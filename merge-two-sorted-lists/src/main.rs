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

fn print_list(l: &Option<Box<ListNode>>) {
    println!("======= Print list =======");
    let mut reflist = l;
    while let Some(node) = reflist {
        if node.next.is_none() {
            print!("{}", node.val);
        } else {
            print!("{} -> ", node.val);
        }
        reflist = &node.next;
    }
    println!();
    println!("===== End Print list =====");
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
        println!("find to insert: {}", node.val);
        tmp = &mut node.next;
    }

    tmp.get_or_insert(data.clone());
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (Some(node1), Some(node2)) => {
            if node1.val > node2.val {
                Some(Box::new(ListNode {
                    val: node2.val,
                    next: merge_two_lists(Some(node1), node2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: node1.val,
                    next: merge_two_lists(node1.next, Some(node2)),
                }))
            }
        }
    }
}

fn main() {
    let list_1 = Option::from(Box::new(ListNode {
        val: 1,
        next: Option::from(Box::new(ListNode {
            val: 2,
            next: Option::from(Box::new(ListNode {
                val: 5,
                next: Option::from(Box::new(ListNode::new(6))),
            })),
        })),
    }));
    let list_2 = Option::from(Box::new(ListNode {
        val: 3,
        next: Option::from(Box::new(ListNode::new(4))),
    }));
    let res = merge_two_lists(list_1, list_2);
    print_list(&res);
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
                    let actual_merged_list = merge_two_lists(actual_list1, actual_list2);
                    let expected_list = create_list(&expected);
                    assert!(compare(actual_merged_list, expected_list), true);
                }
            )*
        };
    }

    test_convert! {
        check_1: (vec![1, 2, 5], vec![3, 4], vec![1, 2, 3, 4, 5]),
        check_2: (vec![1, 1, 1], vec![2, 2], vec![1, 1, 1, 2, 2]),
        check_3: (vec![2, 3, 10], vec![1, 4, 11], vec![1, 2, 3, 4, 10, 11]),
        check_4: (vec![1, 10, 20], vec![2, 2, 2], vec![1, 2, 2, 2, 10, 20]),
    }
}
