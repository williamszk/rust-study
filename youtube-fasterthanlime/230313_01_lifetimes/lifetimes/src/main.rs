use std::fs::read_to_string;


fn main() {
    println!("==================================================================");
    let input = read_to_string("../lines.txt").unwrap();
    // println!("{}", input);
    for line in input.lines() {
        if start_with_capital_letter(line){
            println!("- {}", line);
        }
    }

    println!("==================================================================");
    for line in input.lines().filter(|x| start_with_capital_letter(x)){
            println!("- {}", line);
    }

    println!("==================================================================");
    let mut good_lines=vec![];

    for line in input.lines().filter(|x|start_with_capital_letter(x)){
        good_lines.push(line);
    }
    dbg!(good_lines);

    println!("==================================================================");

    let good_lines: Vec<_> = input
        .lines()
        .filter(|x| start_with_capital_letter(x))
        .collect();

    dbg!(good_lines);
    println!("==================================================================");


}

fn start_with_capital_letter(s: &str) -> bool {
    matches!(s.chars().next(), Some(c) if c.is_uppercase())
}
