/**
 * @brief Server declaration and routing
 * @imports Nickel, STD::Collections, STD::Error
 * @param address as &str
 * @return Result<ListeningServer, Box<StdError>>
 * 
 * Starts the server and routes to destinations depending on input.
 *
 * @author Landon Mote
 * @date 3.14.2016
 */
pub mod serv {
    use nickel::*;
    use std::collections::HashMap;
    use std::error::Error as StdError;

    pub fn start_server(address: &str) -> Result<ListeningServer, Box<StdError>>{
        let mut server = Nickel::new();

        server.utilize(middleware!{ |request|
            println!("Logging Request: {:?}", request.origin.uri);
        });

        server.get("/", middleware! { |_, response|
            let mut data = HashMap::<&str, &str>::new();
            data.insert("title", "Abstract");
            return response.render("assets/home.html", &data);
        });

        server.utilize(StaticFilesHandler::new("assets/"));
        server.utilize(StaticFilesHandler::new("src/"));
        server.listen(address)
    } 
}