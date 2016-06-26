extern crate url;
extern crate hyper;

use std::str;
use std::string::ToString;
use std::io::Read;
use std::str::FromStr;
use self::url::{Url, ParseError};
use self::hyper::client::{Client as HyperClient, IntoUrl};
use self::hyper::header::ContentType;
use ::{Request, Response, Json, ToJson, Error};


pub struct Client {
    uri: String
}

impl Client {
    pub fn new(url: &str) -> Result<Client, &'static str> {
        Ok(Client {uri: url.to_string()})
    }
    pub fn call(&self, method: &str, params: &Option<Json>, id: &i64) -> Result<Option<Json>, &'static str> {
        let request = Request::new("2.0", method, Some(params.to_json()), Some(id.clone()));
        let client  = HyperClient::new();
        let req = client.put(self.uri.into_url().unwrap()).header(ContentType::json())
                    .body(request.to_string().as_bytes())
                    .send();
        match req {
            Ok(mut response) => {
                let mut body: Vec<u8> = Vec::new();
                match response.read_to_end(&mut body) {
                    Ok(size) => match Json::from_str(str::from_utf8(&body).unwrap()) {
                        Ok(json) => {
                            match json.as_object().unwrap().get("result"){
                                Some(result) => {
                                    if result.is_null() {
                                        Ok(None)
                                    } else if result.is_object() 
                                            || result.is_array()
                                            || result.is_u64()
                                            || result.is_i64()
                                            || result.is_f64()
                                            || result.is_string()
                                            || result.is_boolean() {
                                        Ok(Some(result.clone()))
                                    } else {
                                        Ok(None)
                                    }
                                },
                                None => Ok(None)
                            }
                        },
                        Err(_)   => Err("RPC Response To Json Fail.")
                    },
                    Err(_)   => Err("RPC Response Read Fail.")
                }
            },
            Err(_)           => Err("RPC Request Failed.")
        }
    }
}


