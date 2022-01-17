fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }
    println!("some_u8_value is {:?}", some_u8_value);

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
    println!("some_u8_value is {:?}", some_u8_value);
}
