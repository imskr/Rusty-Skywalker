#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
mod utils;


#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_cmd_from_query(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::twitter_url(&cmd),
        _ => utils::google::google_search_from_query(&cmd)
    };
    return Redirect::to(redirect_url);
}

fn rocket() -> rocket::Rocket {
    return rocket::ignite().mount("/", StaticFiles::from("static"))
    .mount("/", routes![search]);
}

fn main() {
    rocket().launch();
}
