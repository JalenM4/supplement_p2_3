/// # Description 
/// This function adds two numbers allocated on the stack
/// # Parameters
/// a - 32-bit integer stored on the stack
/// b - 32-bit integer stored on the stack.
/// # Returns
/// returns the sum of two allocated integers
fn add_stack(a: i32,b: i32) -> i32 {
    a + b
}

fn add_heap(a: Box<i32>, b: Box<i32>) -> i32 {
    *a + *b
}

fn add_mixed(stack_num: i32, heap_num: Box<i32>) -> i32 {
    stack_num + *heap_num
}

mod test;
