fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let s = "hello";
    println!("Original string: {}", s);
    println!("Reversed string: {}", reverse_string(s));
}
