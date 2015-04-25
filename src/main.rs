extern crate civet;
extern crate conduit;
extern crate conduit_router;
extern crate cql_ffi;

use std::collections::HashMap;
use std::io::{self, Cursor};
use std::sync::mpsc::channel;

use cql_ffi::{CassCluster,CassSession};
use civet::{Config, response, Server};
use conduit::{Request, Response};
use conduit_router::{RouteBuilder, RequestParams};

const CONTACT_POINTS:&'static str = "127.0.0.1";


fn prepare(req: &mut Request) -> io::Result<Response> {
    let name = req.params().find("statement").unwrap();
    let bytes = format!("Preparing: {}!", name).into_bytes();
    Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}

fn main() {
	let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let session_future = CassSession::new().connect(cluster).wait();

    match session_future {
        Ok(mut session) => {
			println!("session established");
		    let mut router = RouteBuilder::new();
    		router.get("/prepare/:statement", prepare);
    		let _server = Server::start(Config { port: 8888, threads: 1 }, router);
    		let (_tx, rx) = channel::<()>();
    		rx.recv().unwrap();
		},
   		Err(err) => {panic!("{:?}",err)}

    }
}

//fn hello(_req: &mut Request) -> io::Result<Response> {
//    Ok(response(200, HashMap::new(), "Hello world!".as_bytes()))
//}
