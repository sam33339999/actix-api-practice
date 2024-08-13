use actix_web::{web, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub fn setup_route(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_posts_handle));
    cfg.route("", web::post().to(create_post_handle));
}

pub async fn get_posts_handle() -> impl Responder {
    web::Json(Post {
        id: 1,
        title: "Hello, world!".to_string(),
        body: "Hello, world!".to_string(),
        published: true,
    })
}

pub async fn create_post_handle(data: web::Json<NewPostInput>) -> impl Responder {
    let new_post = NewPost {
        title: &data.title,
        body: &data.body,
    };

    web::Json(create_post(
        &mut crate::establish_connection(),
        new_post.title,
        new_post.body,
    ))
}

// query structure
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPostInput {
    pub title: String,
    pub body: String,
}

// insert structure...
#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
