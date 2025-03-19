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

/// # Description 
/// Function to add two numbers allocated on the heap
/// # Parameters
/// a - 32 bit integer stored in the heap
/// b - 32 bit integer stored in the heap
/// # Returns
/// returnd the sum of the added heap
fn add_heap(a: Box<i32>, b: Box<i32>) -> i32 {
    *a + *b
}

/// # Description 
/// Function to add a stack-allocated number to a heap-allocated number
/// # Parameters 
/// stack_num - a integer allocated on the stack
/// heap_num - a heap allocated integer
/// # Returns 
/// returns a single integer, which is the sum of the stack-allocated number and the dereferenced heap-allocated number
fn add_mixed(stack_num: i32, heap_num: Box<i32>) -> i32 {
    stack_num + *heap_num
}

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

//fn swap<T>(a: T, b: T) -> (T, T) {
//    todo!("not implemented")
//}

//fn max<T: PartialOrd>(a: T, b: T) -> T {
    //todo!("not implemented")
//}

mod test;
