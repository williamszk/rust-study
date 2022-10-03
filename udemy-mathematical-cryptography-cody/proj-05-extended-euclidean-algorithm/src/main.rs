use std::vec;

fn main() {
    println!("Hello, world!");

    let x = 121;
    let y = 88;

    let (a, b) = extended_euclidean_algorithm(x, y);
}

fn extended_euclidean_algorithm(x: i32, y: i32) -> (i32, i32) {
    let a: i32;
    let b: i32;
    let mut q: i32;
    let mut r: i32;
    let mut x_use: i32;
    let mut y_use: i32;

    let mut list_params: Vec<Params> = vec![];

    if x >= y {
        x_use = x;
        y_use = y;
    } else {
        x_use = y;
        y_use = x;
    }

    loop {
        q = x_use / y_use;
        r = x_use % y_use;

        if r == 0 {
            break;
        }

        list_params.push(Params {
            a: x_use,
            b: y_use,
            q: q,
            r: r,
        });

        x_use = y_use;
        y_use = r;
    }

    println!("list_params: {:?}", list_params);

    let len_list = list_params.len();
    let mut i = len_list - 1;

    let r_0: i32 = list_params[i].r;
    let mut a_0: i32 = list_params[i].a;
    let mut b_0: i32 = list_params[i].b;
    let mut q_0: i32 = list_params[i].q;
    let mut p_0: i32 = 1;

    let mut r_1: i32 = list_params[i - 1].r;
    let mut a_1: i32 = list_params[i - 1].a;
    let mut b_1: i32 = list_params[i - 1].b;
    let mut q_1: i32 = list_params[i - 1].q;
    let mut p_1: i32 = 1;

    println!("a = {}", a_0);
    println!("b = {}", b_0);
    println!("p = {}", p_0);
    println!("q = {}", q_0);
    println!("r = {}", r_0);
    println!("");

    loop {
        // r_0 = r_0;
        a_0 = (-1) * a_1;
        p_0 = p_1 * q_0;
        b_0 = b_1; // ( = a_0)
        q_0 = q_1 * q_0 + p_0;

        println!("a = {}", a_0);
        println!("b = {}", b_0);
        println!("p = {}", p_0);
        println!("q = {}", q_0);
        println!("r = {}", r_0);
        println!("");

        i -= 1;
        if i == 0 {
            break;
        }

        r_1 = list_params[i - 1].r;
        a_1 = list_params[i - 1].a;
        b_1 = list_params[i - 1].b;
        q_1 = list_params[i - 1].q;
        p_1 = 1;
    }

    a = p_0;
    b = q_0;

    (a, b)
}

#[derive(Debug)]
struct Params {
    a: i32,
    b: i32,
    q: i32,
    r: i32,
}
