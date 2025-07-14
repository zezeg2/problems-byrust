use crate::Solution;

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

fn collect(node: ListNode) -> Vec<i32> {
    let mut result = Vec::new();
    result.push(node.val);
    if let Some(rear) = node.next {
        result.extend(collect(*rear));
    }
    result
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        
        let mut carry = 0;

        let mut cur1 = l1;
        let mut cur2 = l2;
        
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;
        while cur1.is_some() || cur2.is_some() || carry != 0 {
            let x = cur1.as_ref().map_or(0, |node| node.val);
            let y = cur2.as_ref().map_or(0, |node| node.val);
            let sum = carry + x + y;
            carry = sum / 10;

            *tail = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut tail.as_mut().unwrap().next;

            cur1 = cur1.and_then(|node| node.next);
            cur2 = cur2.and_then(|node| node.next);
        }

        head
    }
}
