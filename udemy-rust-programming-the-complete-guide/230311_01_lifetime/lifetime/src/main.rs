fn main() {
    println!("Hello, world!");

    let _r;

    {
        let x = 5;
        _r = &x;
    }

    // println!("{}", r);

    fn _example<'a>(x: &'a str) -> &'a str {
        x
    }

    fn _example2<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x
    }

}
