fn main() {
    let mut min_stack: MinStack = MinStack::new();
    min_stack.push(1);
    println!("{}", min_stack.get_min());
}

struct MinStack {
    vec: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            vec: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.vec.push(x);
    }
    
    fn pop(&mut self) {
        self.vec.pop();
    }
    
    fn top(&self) -> i32 {
        self.vec[self.vec.len()]
    }
    
    fn get_min(&self) -> i32 {
        let min = self.vec.iter().min();
        *min.unwrap()
    }
}


//  * Your MinStack object will be instantiated and called as such:
//  * let obj = MinStack::new();
//  * obj.push(x);
//  * obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: i32 = obj.get_min();