use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len);

    for post in results {
        println!("{}", post.title);
        println!("------------\n");
        println!("{}", post.body);
    }

}
