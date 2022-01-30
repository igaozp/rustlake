fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 5, y: 4 };
    let both_float = Point { x: 5.0, y: 4.0 };
    let integer_and_float = Point { x: 5.0, y: 4 };

    println!("p.x = {}", integer_and_float.x());

    let p = wont_work.mixup(both_integer);
    println!("p.x = {}, p.y = {}", p.x, p.y);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}