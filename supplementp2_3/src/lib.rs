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

/// # Description
/// Defines a Point3D struct with x, y, and z as f64 values
/// # Parameters
/// x - x coordinate
/// y - y coordinate 
/// z - z coordinate
/// it stores three f64 values representing a point in 3D space.
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

/// # Description
/// Implement addition for Point3D
/// # Parameters
/// self - The first Point3D object.
/// other - The second Point3D object.
/// # Returns
/// Returns a new Point3D instance where each coordinate
use std::ops::Add;
impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// # Description
/// Generic function to swap two variables of the same type
/// # Parameters
/// a - A generic type variable T
/// b - Another generic type variable T
/// # Returns
/// Returns a tuple (T, T), where the order of a and b is swapped.
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

/// # Description
/// Generic function to return the larger of two variables of the same type
/// # Parameters
/// a - A generic type variable that implements PartialOrd (can be compared).
/// b - Another variable of the same type T
/// # Returns
/// Returns the larger of the two variables
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

mod test;
