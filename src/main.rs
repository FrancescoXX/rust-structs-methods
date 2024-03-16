/* Methods in rust are similar to functions, but they are
defined within the context of a struct (or an enum or a trait object).
They are called by instances of the struct, 
and their first parameter is always self,
which represents the instance of the struct.
The following example demonstrates how to define a method
 for the Rectangle struct that calculates the area of a rectangle.
 */

struct Rectangle {
    width: u32,
    height: u32,
}

//impl block is used to define methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //methods with the same name of a field
    fn width(&self) -> bool {
        self.width > 0
    }

    //getter for height
    fn height(&self) -> u32 {
        self.height
    }

    //check if a rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
            (self.width > other.height && self.height > other.width)
    }
}

impl Rectangle {
    //associated function to define a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //create a square
    let square = Rectangle::square(10);

    //calculate the area of the square
    println!("The area of the square is: {}", square.area());
}
