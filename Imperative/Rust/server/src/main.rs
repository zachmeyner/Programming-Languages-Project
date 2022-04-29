#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket::fs::NamedFile;
use rocket::fs::{relative, FileServer};
use rocket::routes;
use rocket::serde::{Deserialize, Serialize};
use rocket_cors::AllowedOrigins;
use rocket_dyn_templates::{handlebars, Template};
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Media {
    location: &'static str,
}

const MYLO_MEDIA: &[Media; 19] = &[
    Media {
        location: "/mylo/01.jpeg",
    },
    Media {
        location: "/mylo/02.jpeg",
    },
    Media {
        location: "/mylo/03.jpg",
    },
    Media {
        location: "/mylo/04.jpeg",
    },
    Media {
        location: "/mylo/05.jpg",
    },
    Media {
        location: "/mylo/06.jpg",
    },
    Media {
        location: "/mylo/07.jpg",
    },
    Media {
        location: "/mylo/08.jpg",
    },
    Media {
        location: "/mylo/09.jpg",
    },
    Media {
        location: "/mylo/10.jpg",
    },
    Media {
        location: "/mylo/11.jpg",
    },
    Media {
        location: "/mylo/12.jpg",
    },
    Media {
        location: "/mylo/13.jpg",
    },
    Media {
        location: "/mylo/14.jpg",
    },
    Media {
        location: "/mylo/15.jpg",
    },
    Media {
        location: "/mylo/16.jpg",
    },
    Media {
        location: "/mylo/17.jpg",
    },
    Media {
        location: "/mylo/18.jpg",
    },
    Media {
        location: "/mylo/19.jpg",
    },
];

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://localhost:8000",
        "http://127.0.0.1:8000",
        "http://0.0.0.0:8000",
    ]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: ["Get", "Post"]
            .into_iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS fairing");

    rocket::build()
        .mount("/photos", FileServer::from("../../media/"))
        .mount("/styles", FileServer::from(relative!("templates")))
        .mount("/", routes![index])
        .attach(Template::fairing())
        .attach(cors)
}

#[get("/")]
pub async fn index() -> Template {
    // println!("{:#?}", context);
    Template::render("index", &MYLO_MEDIA)
}

#[get("/mylo/<img_id>")]
pub async fn photo(img_id: String) -> io::Result<NamedFile> {
    println!("{}", img_id);
    let img = format!("../../images/{}.jpeg", img_id);
    NamedFile::open(img).await
}
