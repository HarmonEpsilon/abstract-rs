pub mod nav;

use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

use nav::*;
use auth::*;
use misc::*;

//Redirect to Home 
#[get("/")]
pub fn take_me_home() -> Redirect {
    return Redirect::to("/home");
}

//Get request for Home, output Home template
#[get("/home")]
pub fn home(mut cook: &Cookies) -> Template {
    let context = HomePage {
        title: "[A] ABSTRACT".to_string(),
        motd: motd(),
        logged_in: is_logged_in(1),
    };

    return Template::render("main/home", &context);
}

//Get request for About page, output About template
#[get("/about")]
pub fn about() -> Template {
    let context = AboutFAQNav {
        title: "[A] ABSTRACT".to_string(),
        nav_about: true,
        logged_in: is_logged_in(1),
    };

    return Template::render("docs/about", &context);
}

//Get request for Register page, output Register template
#[get("/register")]
pub fn sign_up() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "[A] ABSTRACT");
    return Template::render("user/register", &context);
}

//Get request for Sign In page, output Sign In Template
#[get("/signin")]
pub fn sign_in() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "[A] ABSTRACT");
    return Template::render("user/signin", &context);
}

//Get request for Omnibus, outpu Omnibus Template
#[get("/omnibus")]
pub fn omnibus() -> Template {
    let context = TableOfContentsNav {
        title: "[A] ABSTRACT".to_string(),
        nav_toc: true,
        logged_in: is_logged_in(1),
        contents: vec![
                "Preamble",
                "I-Foreward"
            ].iter().map(|s| s.to_string()).collect()
        };
    return Template::render("docs/omnibus", &context);
}

//Get request for Group home page, output Group Page Template
#[get("/group")]
pub fn group() -> Template {
    let context = GroupNav {
        title: "[A] ABSTRACT".to_string(),
        nav_group: true,
        logged_in: is_logged_in(1),
    };

    return Template::render("group/group", &context);
}

//Hook up files such as CSS and JavaScript
#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    return NamedFile::open(Path::new("templates/styles/").join(file)).ok();
}