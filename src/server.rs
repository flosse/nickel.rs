use std::net::ToSocketAddrs;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use hyper::Result as HttpResult;
use hyper::server::{Request, Response, Handler, Listening};
use hyper::server::Server as HyperServer;

use middleware::MiddlewareStack;
use request;
use response;

pub struct Server<D> {
    middleware_stack: MiddlewareStack<D>,
    templates: response::TemplateCache,
    shared_data: D,
}

// FIXME: Any better coherence solutions?
struct ArcServer<D>(Arc<Server<D>>);

impl<D: Sync + Send + 'static> Handler for ArcServer<D> {
    fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, res: Response<'a>) {
        let nickel_req = request::Request::from_internal(req,
                                                         &self.0.shared_data);

        let nickel_res = response::Response::from_internal(res,
                                                           &self.0.templates,
                                                           &self.0.shared_data);

        self.0.middleware_stack.invoke(nickel_req, nickel_res);
    }
}

impl<D: Sync + Send + 'static> Server<D> {
    pub fn new(middleware_stack: MiddlewareStack<D>, data: D) -> Server<D> {
        Server {
            middleware_stack: middleware_stack,
            templates: RwLock::new(HashMap::new()),
            shared_data: data
        }
    }

    pub fn serve<A: ToSocketAddrs>(self, addr: A) -> HttpResult<Listening> {
        let arc = ArcServer(Arc::new(self));
        let server = try!(HyperServer::http(addr));
        server.handle(arc)
    }
}
