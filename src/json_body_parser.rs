use serialize::{Decodable, json};
use request::Request;
use typemap::Key;
use plugin::{Plugin, Pluggable};
use std::io;
use std::io::{Read, ErrorKind};

// Plugin boilerplate
struct JsonBodyParser;
impl Key for JsonBodyParser { type Value = String; }

impl<'mw, 'conn, D> Plugin<Request<'mw, 'conn, D>> for JsonBodyParser {
    type Error = io::Error;

    fn eval(req: &mut Request<D>) -> Result<String, io::Error> {
        let mut s = String::new();
        try!(req.origin.read_to_string(&mut s));
        Ok(s)
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> Result<T, io::Error>;
}

impl<'mw, 'conn, D> JsonBody for Request<'mw, 'conn, D> {
    fn json_as<T: Decodable>(&mut self) -> Result<T, io::Error> {
        self.get_ref::<JsonBodyParser>().and_then(|parsed|
            json::decode::<T>(&*parsed).map_err(|err|
                io::Error::new(ErrorKind::Other, format!("Parse error: {}", err))
            )
        )
    }
}
