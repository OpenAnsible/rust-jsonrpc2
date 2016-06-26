#[warn(non_shorthand_field_patterns)]

extern crate rustc_serialize;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::string::ToString;
pub use rustc_serialize::json::{Json, ToJson, Object};

mod error;
mod request;
mod response;

pub use error::Error;
pub use request::Request;
pub use response::Response;

pub type RpcResult = Result<Json, &'static str>;
pub type RpcHandle = Box<Fn(&Option<Json>)-> RpcResult>;


pub struct JsonRpc {
    methods : BTreeMap<String, RpcHandle>,
    // TODO: Add Shared Memory.
}

impl JsonRpc {
    pub fn new () -> JsonRpc {
        JsonRpc { methods : BTreeMap::new() }
    }
    pub fn register (&mut self, method: &str, handle: RpcHandle) {
        self.methods.insert(method.to_string(), handle);
    }
    pub fn methods(&self) -> &BTreeMap<String, RpcHandle> {
        &self.methods
    }
    pub fn call(&self, request: &Request) -> Response {
        match self.methods.get(&request.method()) {
            Some(func) => {
                // TODO: Support Shared Memory.
                match func(&request.params()) {
                    Ok(result) => {
                        Response::Success{
                            jsonrpc: request.jsonrpc().clone(),
                            result : Some(result),
                            id     : request.id().clone()
                        }
                    },
                    Err(err) => {
                        Response::Error{
                            jsonrpc: request.jsonrpc().clone(),
                            // Error::InternalError
                            error  : Error::ServerError{code: -32000, message: err, data: None},
                            id     : request.id().clone()
                        }
                    }
                }
            },
            None => Response::Error{ jsonrpc: request.jsonrpc().clone(), 
                                     error : Error::MethodNotFound, 
                                     id: request.id().clone() }
        }
    }
}

unsafe impl Send for JsonRpc { }
unsafe impl Sync for JsonRpc { }


mod tests {
    #[warn(non_shorthand_field_patterns)]
    #[warn(unused_imports)]
    use super::{JsonRpc, Error, Request, Response, Json, ToJson, RpcResult};
    use std::str::FromStr;
    use std::string::ToString;

    fn hello(params: &Option<Json>) -> RpcResult {
        Ok("Hello World".to_json())
    }
    fn add(params: &Option<Json>) -> RpcResult {
        match *params {
            Some(ref params) => {
                assert_eq!(params.is_array(), true);
                if params.is_array() {
                    let vec = params.as_array().unwrap();
                    let a = vec[0].as_u64().unwrap();
                    let b = vec[1].as_u64().unwrap();
                    Ok((a+b).to_json())
                } else if params.is_object() {
                    Err("参数错误。")
                } else {
                    Err("参数错误。")
                }
                
            },
            None => Err("参数错误。")
        }
    }
    fn kv(params: &Option<Json>) -> RpcResult {
        assert_eq!(params.is_some(), true);
        let obj = params.clone().unwrap();
        assert_eq!(obj.is_object(),  true);
        match obj.as_object() {
            Some(obj) => {
                let resp = ( obj.get("key").unwrap().as_string().unwrap().to_string() 
                             + ":" 
                             + obj.get("value").unwrap().as_string().unwrap().to_string().as_ref()
                ).to_json();
                Ok(resp)
            },
            None => Err("参数错误。")
        }
    }
    #[test]
    fn test_hello(){
        let mut rpc = JsonRpc::new();
        rpc.register("hello", Box::new(hello));
        let body = "{\"params\": [],       \"jsonrpc\": \"2.0\", \"method\": \"hello\", \"id\": 1}";
        let req  = Request::from_str(&body).unwrap();
        let res  = rpc.call(&req).to_string();
        assert_eq!(&res, "{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":\"Hello World\"}");
    }
    #[test]
    fn test_add(){
        let mut rpc = JsonRpc::new();
        rpc.register("add",   Box::new(add));
        let body = "{\"params\": [10, 20], \"jsonrpc\": \"2.0\", \"method\": \"add\",   \"id\": 2}";
        let req  = Request::from_str(&body).unwrap();
        let res  = rpc.call(&req).to_string();
        assert_eq!(&res, "{\"id\":2,\"jsonrpc\":\"2.0\",\"result\":30}");
    }
    #[test]
    fn test_kv(){
        let mut rpc = JsonRpc::new();
        rpc.register("kv",   Box::new(kv));
        let body = "{\"params\": {\"key\":\"imkey\", \"value\":\"imvalue\"}, \"jsonrpc\": \"2.0\", \"method\": \"kv\",\"id\": 3}";
        let req  = Request::from_str(&body).unwrap();
        let res  = rpc.call(&req).to_string();
        assert_eq!(&res, "{\"id\":3,\"jsonrpc\":\"2.0\",\"result\":\"imkey:imvalue\"}");
    }

}

