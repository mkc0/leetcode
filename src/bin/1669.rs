#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut vals = Vec::new();
        let mut i = 0;
        let mut l1 = list1;
        let mut l2 = list2;
        while let Some(n1) = l1 {
            if i < a || i > b {
                vals.push(n1.val);
            } else if i == a {
                while let Some(n2) = l2 {
                    vals.push(n2.val);
                    l2 = n2.next;
                }
            }
            l1 = n1.next;
            i += 1;
        }
        vals.into_iter().rev().fold(None, |las, x| {
            let mut nd = ListNode::new(x);
            nd.next = las;
            Some(Box::new(nd))
        })
    }
}

#[allow(dead_code)]
struct Solution;

fn main() {}
