//For use with main pages (home, etc...)
#[derive(Serialize)]
pub struct LoggedInNav {
    pub title: String,
    pub logged_in: bool
}

//For use with Omnibus rendering
#[derive(Serialize)]
pub struct TableOfContentsNav {
    pub title: String,
    pub nav_toc: bool,
    pub logged_in: bool,
    pub contents: Vec<String>
}

//For use with About & FAQs rendering
#[derive(Serialize)]
pub struct AboutFAQNav {
    pub title: String,
    pub nav_about: bool,
    pub logged_in: bool,
}

//For use with Group navigation
#[derive(Serialize)]
pub struct GroupNav {
    pub title: String,
    pub nav_group: bool,
    pub logged_in: bool,
}