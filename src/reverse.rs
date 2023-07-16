pub fn reverse(input: &str) -> String {
    let mut rev: String = String::new();
    let mut word = input.chars();
    loop {
        if let Some(next) = word.next() {
            rev = next.to_string() + &rev;
        } else {
            break;
        }
    }
    println!("{}", rev);
    rev
}

fn main() {
    reverse("Hello");
}
