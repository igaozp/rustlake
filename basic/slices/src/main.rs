fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    let word = first_word(&s[..]);
    println!("{}", word);
    let s = "hello world";
    let word = first_word(s);
    println!("{}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}