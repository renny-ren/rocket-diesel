#[macro_use]
extern crate rocket;
use rocket::{fairing::AdHoc, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, get_config])
        .mount(
            "/blog-posts",
            routes![
                get_random_blog_post,
                get_blog_post,
                get_all_blog_posts,
                create_blog_post
            ],
        )
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

#[get("/")]
fn get_all_blog_posts() -> Json<Vec<BlogPost>> {
    Json(vec![
        BlogPost {
            id: 0,
            title: "My First Title".to_string(),
            body: "My First Body".to_string(),
            published: true,
        },
        BlogPost {
            id: 1,
            title: "My Second Title".to_string(),
            body: "My Second Body".to_string(),
            published: true,
        },
    ])
}

#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!("Hello, {} years old named {}!", config.age, config.name)
}

#[post("/", data = "<blog_post>")]
fn create_blog_post(blog_post: Json<BlogPost>) -> Json<BlogPost> {
    blog_post
}
