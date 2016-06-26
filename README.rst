Rust JsonRPC Library
=========================

:Date: 06/26 2016

.. contents::

介绍
-------

Rust JsonRpc Library.

`jsonrpc2 <https://crates.io/crates/jsonrpc2>`_ on `crates.io` .


用例
-------

.. code:: toml

    [dependencies]
    jsonrpc2 = "*"


.. code:: rust

    extern crate jsonrpc2;

    use jsonrpc2::{ 
        JsonRpc, RpcResult, Request, Response, 
        Error as RpcError, Json, ToJson 
    };

    fn hello(params: &Option<Json>) -> RpcResult {
        Ok("Hello World".to_json())
    }
    fn main(){
        let mut rpc = JsonRpc::new();
        rpc.register("hello", Box::new(hello));
        let body = "{\"params\": [],       \"jsonrpc\": \"2.0\", \"method\": \"hello\", \"id\": 1}";
        let req  = Request::from_str(&body).unwrap();
        let res  = rpc.call(&req).to_string();
        
        println!("JsonRpc Response: {:?}", &res);

        assert_eq!(&res, "{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":\"Hello World\"}");
    }


With  Hyper:

.. code:: rust

    extern crate jsonrpc2;

    use std::io::{ copy, Read, Write };
    use std::sync::Arc;
    use std::str::FromStr;

    use hyper::server::{ 
        Server, Request as HyperRequest, 
        Response as HyperResponse, 
        Handler as HyperHandler
    };
    use hyper::method::Method::{ Get, Put, Post };
    use hyper::status::StatusCode; // { Ok, BadRequest, NotFound, MethodNotAllowed };
    
    use jsonrpc2::{ 
        JsonRpc, RpcResult, Request, Response, 
        Error as RpcError, Json, ToJson 
    };

    struct MyHandler {
        rpc: Arc<JsonRpc>
    }
    impl HyperHandler for MyHandler {
        fn handle(&self, mut req: HyperRequest, mut res: HyperResponse) {
            match req.method {
                Post | Put => {
                    let mut body = String::new();
                    match req.read_to_string(&mut body){
                        Ok(body_length) => {
                            let mut res = &mut res.start().unwrap();
                            match Request::from_str(&body) {
                                Ok(rpc_request) => {
                                    let response_content = self.rpc.call(&rpc_request).to_json().to_string();
                                    res.write(response_content.as_bytes()).unwrap();
                                },
                                Err(rpc_error) => {
                                    res.write(rpc_error.to_json().to_string().as_bytes()).unwrap();
                                }
                            }
                        },
                        Err(e) => {
                            let mut res = &mut res.start().unwrap();
                            res.write(e.to_string().as_bytes()).unwrap();
                        }
                    }
                },
                Get => {
                    copy(&mut req, &mut res.start().unwrap()).unwrap();  
                },
                _ => {
                    *res.status_mut() = StatusCode::MethodNotAllowed;
                }
            };
        }
    }
    unsafe impl Send for MyHandler { }
    unsafe impl Sync for MyHandler { }

    fn hello(params: &Option<Json>) -> RpcResult {
        Ok("Hello World".to_json())
    }

    fn main(){
        let mut rpc = JsonRpc::new();
        rpc.register("hello", Box::new(hello));

        let share_rpc = Arc::new(rpc);
        Server::http("0.0.0.0:8000").unwrap().handle( MyHandler{ rpc: share_rpc.clone() } ).unwrap();
    }
    


参考
-------

*   `JSON-RPC <http://www.jsonrpc.org/>`_
*   `JSON-RPC 2.0 Specification <http://www.jsonrpc.org/specification>`_
