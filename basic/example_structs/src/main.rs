fn main() {
    let rec = (30, 50);
    println!("{}", area(rec));

    let rec = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area1(&rec));
    println!("{:#?}", rec);

    let scale = 2;
    let rec = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rec);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
