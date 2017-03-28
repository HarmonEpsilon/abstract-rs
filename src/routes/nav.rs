//For use with Omnibus rendering
#[derive(Serialize)]
pub struct TableOfContentsNav {
    pub title: String,
    pub nav_toc: bool,
    pub contents: Vec<String>
}

//For use with About & FAQs rendering
#[derive(Serialize)]
pub struct AboutFAQNav {
    pub title: String,
    pub nav_about: bool,
}
