use std::fs::read_to_string;
use std::time::Duration;

fn main() {
    let input = read_to_string("../lines.txt").unwrap();


    // dbg!(good_lines);
    let handle = std::thread::spawn(move || {

        let good_lines: Vec<_> = input
            .lines()
            .filter(|x| start_with_capital_letter(x))
            .collect();

        for line in good_lines {
            std::thread::sleep(Duration::from_millis(500));
            println!("Finished processing {:?}", line);
        }
    });

    handle.join().unwrap();
}

fn start_with_capital_letter(s: &str) -> bool {
    matches!(s.chars().next(), Some(c) if c.is_uppercase())
}

