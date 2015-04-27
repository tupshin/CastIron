
extern crate civet;
extern crate conduit;
extern crate conduit_router;
extern crate cql_ffi;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::mpsc::channel;

use cql_ffi::{CassCluster,CassSession,CassError,CassPrepared};
use civet::{Config, response, Server};
use conduit::{Request, Response};
use conduit_router::{RouteBuilder, RequestParams};

const CONTACT_POINTS:&'static str = "127.0.0.1";

fn main() {
	let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let session_future = CassSession::new().connect(cluster).wait();
    
    let all_prepared:Vec<CassPrepared> = Vec::new();

    match session_future {
   		Err(err) => panic!("{:?}",err),
        Ok(session) => {
			println!("session established");
			
		    let mut router = RouteBuilder::new();
    		router.get("/prepare/:statement", |r: &mut Request| prepare(r, &session, &all_prepared));
    		router.get("/execute/:statement_id", |r: &mut Request| execute(r, &session, &all_prepared));
    		let _server = Server::start(Config { port: 8888, threads: 1 }, router);
    		let (_tx, rx) = channel::<()>();
    		rx.recv().unwrap();
		}
    }
}

fn prepare(req: &mut Request, session: &CassSession, all_prepared:&Vec<CassPrepared>) -> Result<Response,CassError> {
	let statement = req.params().find("statement").unwrap();
	let prepared:CassPrepared = try!(try!(session.prepare(statement)).wait());
    let bytes = format!("Preparing: {}!", statement).into_bytes();
     Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}

fn execute(req: &mut Request, session: &CassSession, all_prepared:&Vec<CassPrepared>) -> Result<Response,CassError> {
	let statement = req.params().find("statement").unwrap();
	let prepared:CassPrepared = try!(try!(session.prepare(statement)).wait());
	//prepared.
    let bytes = format!("Preparing: {}!", statement).into_bytes();
     Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}

