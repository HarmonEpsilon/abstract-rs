/**
 * @brief Login functions
 * @imports Nickel, STD::Collections, STD::Error
 * @param address as &str
 * @return Result<ListeningServer, Box<StdError>>
 * 
 * Starts the server and routes to destinations depending on input.
 *
 * @author Landon Mote
 * @date 3.14.2016
 */
pub mod login {
    use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};
    use mysql as database;
    use bson::{Bson, Document};

    pub struct User {
        id: u32,
        username: Option<String>,
        password: Option<String>,
        email: Option<String>
    }
}