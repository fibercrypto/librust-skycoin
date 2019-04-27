#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate hyper;
extern crate futures;
extern crate tokio_core;

//pub use self::default_api::{ DefaultApi, DefaultApiClient };

pub mod apis;
pub mod models;
pub struct HttpConnector;

//pub mod std;

#[cfg(test)]
mod tests {
//    use apis::DefaultApiClient;
//    use apis::client::APIClient;
    use apis;
//    use apis::default_api;
    use apis::{DefaultApiClient, DefaultApi};
    use tokio_core::reactor::Handle;
    use hyper::client::HttpConnector;
    use std::rc::Rc;
    use apis::configuration::Configuration;

//    use hyper::client::HttpConnector;
//    use apis::APIClient;
//    use std::ptr::null;

    #[test]
    fn exploration() {
        let core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = hyper::Client::configure()
            .keep_alive(true)
            .build(&handle);


        let context = DefaultApiClient::new(Rc::new(
            Configuration::new(client)));
//
//        println!("{}", );
        let version = context.version();
//        println!("{}", context.version());
        assert_eq!(2 + 2, 4);
    }
}
