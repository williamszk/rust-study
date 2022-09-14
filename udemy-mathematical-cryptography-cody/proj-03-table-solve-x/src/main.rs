// In this project we take the last one proj-02 and try to build a general
// way to build a table that can receive equations and functions
// to fill up the values that are solutions to the problem
// of linear congruence

fn main() {
    // string
    // function

    // my vector of Column info
    let columns: Vec<Column> = vec![
        Column {
            equation: String::from("x"),
            function: |x: i32| -> String { x.to_string() },
        },
        Column {
            equation: String::from("2x ≡ 2 (mod 4)"),
            function: |x: i32| -> String { draw_result((2 * x - 2) % 4 == 0) },
        },
        Column {
            equation: String::from("x ≡ 1 (mod 4)"),
            function: |x: i32| -> String { draw_result((x - 1) % 4 == 0) },
        },
        Column {
            equation: String::from("4x ≡ 4 (mod 4)"),
            function: |x: i32| -> String { draw_result((4 * x - 4) % 4 == 0) },
        },
    ];

    draw_table(&columns, -50, 51);
    println!("\n\n");

    let columns2: Vec<Column> = vec![
        Column {
            equation: String::from("x"),
            function: |x: i32| -> String { x.to_string() },
        },
        Column {
            equation: String::from("2x ≡ 2 (mod 7)"),
            function: |x: i32| -> String { draw_result((2 * x - 2) % 7 == 0) },
        },
        Column {
            equation: String::from("x ≡ 1 (mod 7)"),
            function: |x: i32| -> String { draw_result((x - 1) % 7 == 0) },
        },
        Column {
            equation: String::from("4x ≡ 4 (mod 7)"),
            function: |x: i32| -> String { draw_result((4 * x - 4) % 7 == 0) },
        },
        Column {
            equation: String::from("3x ≡ 3 (mod 7)"),
            function: |x: i32| -> String { draw_result((3 * x - 3) % 7 == 0) },
        },
    ];

    draw_table(&columns2, -10, 51);
    println!("\n\n");

    let columns3: Vec<Column> = vec![
        Column {
            equation: String::from("x"),
            function: |x: i32| -> String { x.to_string() },
        },
        Column {
            equation: String::from("2x ≡ 2 (mod 13)"),
            function: |x: i32| -> String { draw_result((2 * x - 2) % 13 == 0) },
        },
        Column {
            equation: String::from("x ≡ 1 (mod 13)"),
            function: |x: i32| -> String { draw_result((x - 1) % 13 == 0) },
        },
        Column {
            equation: String::from("4x ≡ 4 (mod 13)"),
            function: |x: i32| -> String { draw_result((4*x - 4) % 13 == 0) },
        },
    ];

    draw_table(&columns3, -10, 51);
    println!("\n\n");












}

fn draw_result(result: bool) -> String {
    if result {
        String::from("✓")
    } else {
        String::from("-")
    }
}

fn draw_table(columns: &Vec<Column>, min:i32, max:i32) {
    println!("{}", draw_horizontal_line(&columns));
    println!("{}", draw_column_headers(&columns));
    println!("{}", draw_horizontal_line(&columns));
    for x in min..max {
        for column in columns {
            print!("{}", draw_content(column, x));
        }
        println!("|");
    }
    println!("{}", draw_horizontal_line(&columns));
}

fn draw_content(column: &Column, x: i32) -> String {
    let mut return_string = String::from("");
    return_string.push_str("|");

    return_string = format!(
        "{}{}",
        return_string,
        format!(
            "{: ^width$}",
            (column.function)(x),
            width = column.equation.len() + 3
        )
    );

    return_string
}

fn draw_column_headers(columns: &Vec<Column>) -> String {
    let mut return_string = String::from("");
    return_string.push_str("|");

    for column in columns {
        return_string = format!(
            "{}{}",
            return_string,
            format!(
                "{: ^width$}",
                column.equation,
                width = column.equation.len() + 3
            )
        );
        return_string.push_str("|");
    }

    return_string
}

fn draw_horizontal_line(columns: &Vec<Column>) -> String {
    let mut return_string = String::from("");
    return_string.push_str("+");

    for column in columns {
        return_string = format!(
            "{}{}",
            return_string,
            format!("{:-^width$}", "", width = column.equation.len() + 3)
        );
        return_string.push_str("+");
    }

    return_string
}

struct Column {
    equation: String,
    function: fn(x: i32) -> String,
}

// how to use a function as a field of a struct in rust?
// https://stackoverflow.com/questions/37370120/right-way-to-have-function-pointers-in-struct

// how to build anonymous functions in rust?
// https://medium.com/coding-rust/best-explanation-of-closure-in-rust-2b20210eba53
