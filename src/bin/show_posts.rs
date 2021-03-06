extern crate xoxo_lib;
extern crate diesel;

use xoxo_lib::db::models::*;
use xoxo_lib::db::common::*;

use diesel::prelude::*;

fn main() {
    use xoxo_lib::db::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
