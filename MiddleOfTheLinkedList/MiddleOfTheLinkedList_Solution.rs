// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_iter = head.clone();
        
        let mut counter : f64 = 0.0;
        loop {
            match head_iter {
                Some(x) => {
                    counter += 1.0;
                    head_iter = x.next;
                },
                None => break
            }
        }
        
        let mut middle = 0;
        // println!("{}", counter);
        if (counter % 2.0 == 0.0) {
            middle = ((counter as i64) / 2) + 1;
        } else {
            middle = ((counter / 2.0).ceil() as i64);
        }
        
        
        // println!("{}", middle);
        head_iter = head.clone();
        for i in 1..middle {
            // println!("{:?}", head_iter);
            head_iter = head_iter.unwrap().next;
        }
        
        head_iter
    }
}