fn main() {
    println!("Hello, world!");

    let mut _s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    println!("--------------------------------------------------------------");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    println!("--------------------------------------------------------------");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    println!("--------------------------------------------------------------");
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    println!("--------------------------------------------------------------");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3={}", s3);
    println!("s2={}", s2);
    // println!("s1={}", s1); // this will crash

    println!("--------------------------------------------------------------");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s={}", s);

    println!("--------------------------------------------------------------");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s={}", s);

    println!("--------------------------------------------------------------");
    let len = String::from("Hola").len();
    println!("the len of Hola: {}", len);

    let len = String::from("Здравствуйте").len();
    println!("the len of Здравствуйте: {}", len);

    println!("--------------------------------------------------------------");
    let word = ['न', 'म', 'स', '्', 'त', 'े'];
    println!("{:?}", word);
    // the char type represent a grapheme cluster, a letter
    let word = ["न", "म", "स्", "ते"];
    println!("{:?}", word);

    println!("--------------------------------------------------------------");
    // Unicode scalar values
    // use .chars() to get the unicode character, not the bytes
    println!("Here we print the chars, but not the grapheme clusters, note that accent marks are treated separetely.");
    println!("Original: नमस्ते");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("--------------------------------------------------------------");
    println!("Returning the bytes of the string");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    println!("--------------------------------------------------------------");
    println!("An experiment with grapheme clusters");
    println!("Original: áòçárvorecocô");
    for c in "áòçárvorecocô".chars() {
        println!("{}", c);
    }
    // Using accents in this case does not behave like different parts
    // of a grapheme cluster
}
