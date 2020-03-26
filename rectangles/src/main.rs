#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, rect : Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size : u32) -> Rectangle {
        Rectangle{width : size, height: size}
    }
}

fn main() {
    let rect = Rectangle{
        width : 30,
        height : 50
    };

    let rect1 = Rectangle{
        width : 10,
        height : 20
    };

    println!("The area of rectangle is {}", rect.area());
    println!("rect hold rect1: {}", rect.can_hold(rect1));
    println!("The area of square is {}", Rectangle::square(15).area());
}