fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rec.area());

    if rec.width() {
        println!("The rectangle has nonzero width; it is {}", rec.width);
    }

    let rec1 = Rectangle {
        width: 10,
        height: 40
    };
    println!("Can rec hold rec1? {}", rec.can_hold(&rec1));

    let sq = Rectangle::square(2);
    println!("Can rec hold sq? {}", rec.can_hold(&sq));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}