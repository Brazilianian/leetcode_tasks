use std::fs::metadata;

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(f), Some(s)) => {
                if f.val < s.val {
                    Option::from(Box::from(ListNode {
                        val: f.val,
                        next: Solution::merge_two_lists(f.next, Some(s)),
                    }
                    ))
                } else {
                    Option::from(Box::from(ListNode {
                        val: s.val,
                        next: Solution::merge_two_lists(Some(f), s.next),
                    }))
                }
            }
        };
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::merge_two_sorted_lists::{ListNode, Solution};

    #[test]
    fn should_merge_two_lists() {
        let first_list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                })),
            })),
        }));

        let second_list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: None
                            }))
                        }))
                    }))
                })),
            })),
        }));

        assert_eq!(Solution::merge_two_lists(first_list, second_list), expected)
    }
}
