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

fn insert_node(l: &mut Option<Box<ListNode>>, data: Box<ListNode>) {
    let mut tmp = l;
    while let Some(node) = tmp {
        println!("find to insert: {}", node.val);
        tmp = &mut node.next;
    }

    tmp.get_or_insert(data);
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = Option::default();

    let mut reflist1 = l1.clone();
    let mut reflist2 = l2.clone();

    loop {
        if let Some(node1) = reflist1.clone() {
            if let Some(node2) = reflist2.clone() {
                println!("compare: {} - {}", node1.val, node2.val);
                if node1.val > node2.val {
                    reflist2 = node2.next;
                    let new_node = Box::new(ListNode::new(node2.val));

                    if root.is_none() {
                        root = Option::from(new_node);
                    } else {
                        insert_node(&mut root, new_node);
                    }
                } else {
                    reflist1 = node1.next;
                    let new_node = Box::new(ListNode::new(node1.val));

                    if root.is_none() {
                        root = Option::from(new_node);
                    } else {
                        insert_node(&mut root, new_node);
                    }
                }
            } else {
                println!("End of second list. Append rest of the first list");
                insert_node(&mut root, node1);
                break;
            }
        } else {
            println!("End of first list. Append rest of the second list");
            break;
        }
    }

    root
}

fn main() {
    let list_1 = Option::from(Box::new(ListNode {
        val: 1,
        next: Option::from(Box::new(ListNode {
            val: 3,
            next: Option::from(Box::new(ListNode {
                val: 5,
                next: Option::from(Box::new(ListNode::new(6))),
            })),
        })),
    }));
    let list_2 = Option::from(Box::new(ListNode {
        val: 2,
        next: Option::from(Box::new(ListNode::new(4))),
    }));
    let res = merge_two_lists(list_1, list_2);
    let mut reflist3 = res;
    while let Some(node3) = reflist3 {
        print!("{} -> ", node3.val);
        reflist3 = node3.next;
    }
    println!();
}
