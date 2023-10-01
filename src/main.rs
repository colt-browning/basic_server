#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use rocket::form::Form;
use rocket_dyn_templates::Template;
use std::fs;

const F: &'static str = "content.txt";

#[derive(Debug, FromForm)]
struct Submit<'v> {
    content: &'v str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context!(
        content: fs::read_to_string(F).expect("Should be able to read the file"),
    ))
}

#[post("/", data = "<form>")]
fn submit<'r>(form: Form<Submit<'r>>) -> Template {
    let data = form.content;
    fs::write(F, data).expect("Should be able to write to the file");
    println!("content: {:#?}", data);
    index()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
}
