use std::fs::read_to_string;
// use std::time::Duration;

fn main() {
    let input = read_to_string("../lines.txt").unwrap();
    dbg!(input.as_ptr());

    std::thread::spawn(move ||{
        dbg!(input.as_ptr());
    })
    .join()
    .unwrap();

}

#[allow(unused)]
fn start_with_capital_letter(s: &str) -> bool {
    matches!(s.chars().next(), Some(c) if c.is_uppercase())
}

