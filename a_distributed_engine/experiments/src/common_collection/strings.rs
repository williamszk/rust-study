pub fn _main() {
    _create_literal_string_from_slice();
    _concat_with_plus_operator();
}

fn _create_literal_string_from_slice() {
    let data = "initial contents";
    let _s = data.to_string();
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let _hello = String::from("�����������");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("������");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // -------------------------------------------------------------------------
    let mut s = String::from("foo");
    s.push_str("bar");

    // -------------------------------------------------------------------------
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // note that push_str will not take ownership of the parameter
    println!("s2 is {}", s2);

    // -------------------------------------------------------------------------
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}

fn _concat_with_plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // let _s3 = s1.clone() + &s2; // cloning could solve the problem
    // println!("{}", s1);
}
