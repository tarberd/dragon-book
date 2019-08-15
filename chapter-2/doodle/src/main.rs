fn main() {
    println!("Hello, world!");

    let input = "5 + 9";

    let mut tokens = vec![];

    for character in input.chars() {
        if character.is_ascii_digit() {
            tokens.push("digit");
        }

        if character == '+' {
            tokens.push("+");
        }
    }

    println!("{:?}", tokens);
}
