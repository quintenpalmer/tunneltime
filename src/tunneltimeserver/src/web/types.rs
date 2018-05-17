use futures::future::Future;

use hyper;
use hyper::server::Response;

pub type ResponseFuture = Box<Future<Item = Response, Error = hyper::Error>>;
