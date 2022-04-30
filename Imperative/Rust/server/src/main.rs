#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket::fs::{relative, FileServer};
use rocket::routes;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::{handlebars, Template};

// Serialized Structs are often used for data passing
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Media {
    pic: bool,
    location: &'static str,
    date: &'static str,
    id: usize,
    desc: &'static str,
}

// In actual implementation this would be photo paths stored in a DB.
// I am way too lazy to set that up.
static MYLO_MEDIA: &[Media] = &[
    Media {
        pic: true,
        location: "mylo/01.jpeg",
        date: "March 2, 2019",
        id: 0,
        desc:
            "I thought he looked really funny in this. One of my favorite Mylo photos of all time.",
    },
    Media {
        pic: true,
        location: "mylo/02.jpeg",
        date: "March 1, 2019",
        id: 1,
        desc: "I don't know much about this one. I thought it was cute the way he laid against my leg.",
    },
    Media {
        pic: true,
        location: "mylo/03.jpg",
        date: "September 3, 2018",
        id: 2,
        desc: "Mylo really likes sleeping near the shoes during weekdays. I do not know why.",
    },
    Media {
        pic: true,
        location: "mylo/04.jpeg",
        date: "July 8, 2018",
        id: 3,
        desc: "He hates the basket, but he decided on this day to make peace with the basked.",
    },
    Media {
        pic: true,
        location: "mylo/05.jpg",
        date: "July 21, 2018",
        id: 4,
        desc: "Another time I thought he looked really cute laying up against my leg.",
    },
    Media {
        pic: true,
        location: "mylo/06.jpg",
        date: "June 7, 2018",
        id: 5,
        desc: "He enjoys laying in the during weekdays when no one is home. THe basked was in his way but he made it work.",
    },
    Media {
        pic: true,
        location: "mylo/07.jpg",
        date: "July 7, 2021",
        id: 6,
        desc: "Mylo really likes sleeping near the shoes during weekdays. I do not know why.",
    },
    Media {
        pic: true,
        location: "mylo/08.jpg",
        date: "May 21, 2018",
        id: 7,
        desc: "He was feeling a little sick around this time and would come upstairs to where I used to my computer to stay near me.",
    },
    Media {
        pic: true,
        location: "mylo/09.jpg",
        date: "Mar 14, 2018",
        id: 8,
        desc: "We have to keep blankets and pillow on the chair when he's alone cause he likes to tear up the chair. He's taken advantage of that to his comfort here.",
    },
    Media {
        pic: true,
        location: "mylo/10.jpg",
        date: "August 3, 2021",
        id: 9,
        desc: "He likes to lay in the sunlight during weekdays when no one is home. He does not care if anything is in his way."
    },
    Media {
        pic: true,
        location: "mylo/11.jpg",
        date: "July 27, 2021",
        id: 10,
        desc: "He was sleeping next to the socks."
    },
    Media {
        pic: true,
        location: "mylo/12.jpg",
        date: "June 26, 2021",
        id: 11,
        desc: "He is very sleepy during the day. He has the whole night to sleep and always ends up like this."
    },
    Media {
        pic: true,
        location: "mylo/13.jpg",
        date: "April 27, 2021",
        id: 12,
        desc: "Once again leveraging the pillow and blanket on chair."
    },
    Media {
        pic: true,
        location: "mylo/15.jpg",
        date: "June 29, 2019",
        id: 13,
        desc: "He likes to sleep on shoes."
    },
    Media {
        pic: true,
        location: "mylo/16.jpg",
        date: "May 15, 2019",
        id: 14,
        desc: "He's very cute while yawning."
    },
    Media {
        pic: true,
        location: "mylo/17.jpg",
        date: "May 15, 2019",
        id: 15,
        desc: "He does not often use his bed, but when he does he makes sure to stare at you to let you know that he could be laying down with you istead."
    },
    Media {
        pic: true,
        location: "mylo/18.jpg",
        date: "April 14, 2019",
        id: 16,
        desc: "He likes to sleep on peoples chests."
    },
    Media {
        pic: true,
        location: "mylo/19.jpg",
        date: "April 8, 2019",
        id: 17,
        desc: "There was no sunlight for him to lay in on this day, so he just slept in the bed."
    },
    Media {
        pic: true,
        location: "mylo/20.jpg",
        date: "February 26, 2021",
        id: 18,
        desc: "He looks very comfy as he lays with all these blanks around him."
    },
    Media {
        pic: false,
        location: "mylo/01.mp4",
        date: "4/30/2018",
        id: 19,
        desc: "He would always get very excited when I got home from school. He was often much more excited than this when I would get home."
    },
    Media {
        pic: false,
        location: "mylo/02.mp4",
        date: "4/12/2019",
        id: 20,
        desc: "He makes funny sounds when he likes the way that you are petting him."
    },
];

// #[launch] tells the rocket framework what the server should be running.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/photos", FileServer::from("../../media/"))
        .mount("/styles", FileServer::from(relative!("templates")))
        .mount("/", routes![index, media_page])
        .attach(Template::fairing())
}

// Render index template
#[get("/")]
pub async fn index() -> Template {
    // println!("{:#?}", context);
    Template::render("index", &MYLO_MEDIA)
}

// Render media page template
#[get("/dispmedia/<id>")]
pub async fn media_page(id: usize) -> Template {
    Template::render("picture", &MYLO_MEDIA[id])
}
