fn main() {
    let len_x = 6;

    let equation1 = "2x ≡ 2 (mod 4)";
    let len_eq1 = equation1.len();

    let equation2 = "x ≡ 1 (mod 4)";
    let len_eq2 = equation2.len();

    let equation3 = "4x ≡ 4 (mod 4)";
    let len_eq3 = equation3.len();

    print!("+");
    print!("{}+", format!("{:-^width$}", "", width = len_x));
    print!("{}+", format!("{:->width$}", "", width = len_eq1 + 3));
    print!("{}+", format!("{:->width$}", "", width = len_eq2 + 3));
    print!("{}+", format!("{:->width$}", "", width = len_eq3 + 3));
    println!("");

    print!("|");
    print!("{}|", format!("{: ^width$}", "x", width = len_x));
    print!(
        "{}|",
        format!("{: ^width$}", equation1, width = len_eq1 + 3)
    );
    print!(
        "{}|",
        format!("{: ^width$}", equation2, width = len_eq2 + 3)
    );
    print!(
        "{}|",
        format!("{: ^width$}", equation3, width = len_eq3 + 3)
    );
    println!("");

    print!("+");
    print!("{}+", format!("{:-^width$}", "", width = len_x));
    print!("{}+", format!("{:->width$}", "", width = len_eq1 + 3));
    print!("{}+", format!("{:->width$}", "", width = len_eq2 + 3));
    print!("{}+", format!("{:->width$}", "", width = len_eq3 + 3));
    println!("");

    for x in -50..51 {
        print!("|");
        print!("{}|", format!("{: ^width$}", x, width = len_x));

        if (2 * x - 2) % 4 == 0 {
            print!("{}|", format!("{: ^width$}", "✓", width = len_eq1 + 3));
        } else {
            print!("{}|", format!("{: ^width$}", "-", width = len_eq1 + 3));
        }

        if (x - 1) % 4 == 0 {
            print!("{}|", format!("{: ^width$}", "✓", width = len_eq2 + 3));
        } else {
            print!("{}|", format!("{: ^width$}", "-", width = len_eq2 + 3));
        }

        if (4*x - 4) % 4 == 0 {
            print!("{}|", format!("{: ^width$}", "✓", width = len_eq3 + 3));
        } else {
            print!("{}|", format!("{: ^width$}", "-", width = len_eq3 + 3));
        }

        println!("");
    }

    print!("+");
    print!("{}+", format!("{:-^width$}", "", width = len_x));
    print!("{}+", format!("{:->width$}", "", width = len_eq1 + 3));
    print!("{}+", format!("{:->width$}", "", width = len_eq2 + 3));
    print!("{}+", format!("{:->width$}", "", width = len_eq3 + 3));
    println!("");
}
