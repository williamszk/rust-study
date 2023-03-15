fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Error");
    let important_exc = ImportantExcerpt{
        part: first_sentence
    };
}
// A problem may appear if the struct lives more than the reference that it makes
// the &str, because if the struct lives longer than if we access part, we'll 
// access an invalid memory.
// We use generic lifetime annotations to explicitly tell the borrow checker and 
// programmer that the lifetime of the struct is the same as the lifetime of its
// reference.
struct ImportantExcerpt<'a> {
    part: &'a str
}

