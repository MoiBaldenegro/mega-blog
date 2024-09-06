extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::post::Post;
// se models::post::NewPost;
fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);

    let mut conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    use schema::posts::dsl::*;

    // select * from 
    let posts_res = posts.load::<Post>(&mut conn).expect("Error loading posts");
/* 
    use schema::posts; // Asegúrate de importar el módulo de la tabla

       // Insert a new post
       let new_post = NewPost {
        title: "Moises POST 3",
        slug: "my-new-moises-post 3",
        body: "This is the content for Moises of my new post 3.",
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&mut conn)
        .expect("Error inserting new post");
*/
    
    for post in posts_res {
        println!("{:?}", post.title);
        println!("{:?}", post.slug);
        println!("{:?}", post.body);
        
    }
    print!("Fin de la ejecucion");
}



