pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return match (l1, l2) {
            (None, None) => {
                None
            }
            (Some(v), None) | (None, Some(v)) => {
                Some(v)
            }
            (Some(f), Some(s)) => {
                let sum = f.val + s.val;
                if sum >= 10 {
                    let carry = Option::from(Box::new(ListNode::new(sum / 10)));
                    Option::from(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(Solution::add_two_numbers(carry, f.next), s.next),
                    }))
                } else {
                    Option::from(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(f.next, s.next),
                    }))
                }
            }
        };
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_add_each_node_of_list() {
        let first_list = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None
                })),
            })),
        }));

        let second_list = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None
                    }))
                })),
            })),
        }));

        assert_eq!(Solution::add_two_numbers(first_list, second_list), expected)
    }
}
