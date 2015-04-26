#![deny(warnings)]
extern crate civet;
extern crate conduit;
extern crate conduit_router;
extern crate cql_ffi;

extern crate iron;
//extern crate serde;


use std::collections::HashMap;
use std::io::{self, Cursor};
use std::sync::mpsc::channel;

use cql_ffi::{CassCluster,CassSession};
use civet::{Config, response, Server};
use conduit_router::{RouteBuilder, RequestParams};

extern crate router;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

const CONTACT_POINTS:&'static str = "127.0.0.1";

fn main() {
    
    let mut router = Router::new();
	let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let s1 = CassSession::new().connect(cluster).wait().unwrap();
	let s2 = "foo";
    router.get("/", move |r: &mut Request| prepare1(r, s1));
    router.get("/", move |r: &mut Request| prepare2(r, s2));
    

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request, greeting: &str) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        println!("{}", payload);
        Ok(Response::with((status::Ok,greeting)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}


fn prepare1(request: &mut Request, session: CassSession) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    println!("{}", payload);
    let greeting = "foo";
    Ok(Response::with((status::Ok,greeting)))
}

fn prepare2(request: &mut Request, session: &str) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    println!("{}", payload);
    let greeting = "foo";
    Ok(Response::with((status::Ok,greeting)))
}
//
//
//fn main() {
//	let cluster = &CassCluster::new()
//                        .set_contact_points(CONTACT_POINTS).unwrap()
//                        .set_load_balance_round_robin().unwrap();
//
//    let session_future = CassSession::new().connect(cluster).wait();
//
//    match session_future {
//        Ok(mut session) => {
//			println!("session established");
//			
//			let prep = |req: &mut Request| prepare(req, session);
//
//		    let mut router = RouteBuilder::new();
//    		router.get("/prepare/:statement", |r: &mut Request| prepare(r, session));
//    		let _server = Server::start(Config { port: 8888, threads: 1 }, router);
//    		let (_tx, rx) = channel::<()>();
//    		rx.recv().unwrap();
//		},
//   		Err(err) => {panic!("{:?}",err)}
//
//    }
//}

