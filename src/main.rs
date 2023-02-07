#[macro_use]
extern crate rocket;
// use diesel::{prelude::*, table, Insertable, Queryable};
use rocket::{fairing::AdHoc, serde::json::Json, State};
// use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

// fn main() {
//   println!("Hello, world!");
// }

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .mount("/", routes![index])
        .mount("/blog-posts", routes![get_random_blog_post, get_blog_post])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost {
        id: 1,
        title: "My first post".to_string(),
        body: "This is my first post".to_string(),
        published: true,
    })
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(BlogPost {
        id,
        title: "Some title".to_string(),
        body: "Some body".to_string(),
        published: true,
    })
}

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}
