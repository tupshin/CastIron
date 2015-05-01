
extern crate civet;
extern crate conduit;
extern crate conduit_router;
extern crate cql_ffi;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
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
    

    match session_future {
   		Err(err) => panic!("{:?}",err),
        Ok(session) => {
		    let all_prepared1:Arc<Vec<CassPrepared>> = Arc::new(Vec::new());
		    let all_prepared2:Arc<Vec<CassPrepared>> = all_prepared1.clone();
		    
			println!("session established");
			let session = Arc::new(session);
			let prepare_session = session.clone();
			let execute_session = session.clone();
		    let mut router = RouteBuilder::new();
		    
		    router.get("/prepare/:statement", move |r: &mut Request| prepare(r, &prepare_session, &all_prepared1));
    		router.get("/execute/:statement_id", move |r: &mut Request| execute(r, &execute_session, &all_prepared2));
    		
    		let _server = Server::start(Config { port: 8888, threads: 1 }, router);
    		let (_tx, rx) = channel::<()>();
    		rx.recv().unwrap();
		}
    }
}

fn prepare(req: &mut Request, session: &Arc<CassSession>, all_prepared: &Arc<Vec<CassPrepared>>) -> Result<Response,CassError> {
	let statement = req.params().find("statement").unwrap();
	let prepared:CassPrepared = try!(try!(session.prepare(statement)).wait());
    let bytes = format!("Preparing: {}!", statement).into_bytes();
     Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}

fn execute(req: &mut Request, session: &CassSession, all_prepared: &Arc<Vec<CassPrepared>>) -> Result<Response,CassError> {
	let statement = req.params().find("statement").unwrap();
	let prepared:CassPrepared = try!(try!(session.prepare(statement)).wait());
	//prepared.
    let bytes = format!("Preparing: {}!", statement).into_bytes();
     Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}

