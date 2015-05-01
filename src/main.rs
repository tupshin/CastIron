
extern crate civet;
extern crate conduit;
extern crate conduit_router;
extern crate cql_ffi;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::mpsc::channel;

use cql_ffi::{CassCluster, CassSession, CassError, CassPrepared};
use civet::{Config, response, Server};
use conduit::{Request, Response};
use conduit_router::{RouteBuilder, RequestParams};

struct CassEnv {
    pub session:CassSession,
    pub prepared:Vec<CassPrepared>
}

const CONTACT_POINTS:&'static str = "127.0.0.1";

fn main() {
    let cluster = &CassCluster::new()
                        .set_contact_points(CONTACT_POINTS).unwrap()
                        .set_load_balance_round_robin().unwrap();

    let session_future = CassSession::new().connect(cluster).wait();


    match session_future {
        Err(err) => panic!("{:?}",err),
        Ok(session) => {
            let cass_env = Arc::new(CassEnv{session:session,prepared:vec!()});
            println!("session established");
            let mut router = RouteBuilder::new();
               
            let prepare_env = cass_env.clone();
            let execute_env = cass_env.clone();
            router.get("/prepare/:statement", move |r: &mut Request| prepare(r, &prepare_env));
            router.get("/execute/:statement_id", move |r: &mut Request| execute(r, &execute_env));
            let _server = Server::start(Config { port: 8888, threads: 1 }, router);
            let (_tx, rx) = channel::<()>();
            rx.recv().unwrap();
        }
    }
}

fn prepare(req: &mut Request, cass_env: &CassEnv) -> Result<Response, CassError> {
    let statement = req.params().find("statement").unwrap();
    let prepared = try!(try!(cass_env.session.prepare(statement)).wait());
    let bytes = format!("Preparing: {}!", statement).into_bytes();
    Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}

fn execute(req: &mut Request, cass_env: &CassEnv) -> Result<Response, CassError> {
    let statement = req.params().find("statement").unwrap();
    let prepared = try!(try!(cass_env.session.prepare(statement)).wait());
    let bytes = format!("Preparing: {}!", statement).into_bytes();
    Ok(response(200, HashMap::new(), Cursor::new(bytes)))
}
