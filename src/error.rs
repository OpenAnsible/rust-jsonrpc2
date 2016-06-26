
use std::collections::BTreeMap;
// use std::str::FromStr;
use std::string::ToString;
use ::{Json, ToJson, Object};


#[derive(Debug, Clone)]
pub enum Error {
    ParseError,                             // -32700
    InvalidRequest,                         // -32600
    MethodNotFound,                         // -32601
    InvalidParams,                          // -32602
    InternalError,                          // -32603
    ServerError( i64,  String,  Option<Json>), // -32000 to -32099
    Unregister ( i64,  String,  Option<Json>)  // other code.
}

impl ToJson for Error {
    fn to_json(&self) -> Json {
        let make_json = |code: i64, message: String, data: Option<Json>| -> Json {
            let mut json = BTreeMap::new();
            json.insert("code".to_string(),    code.to_json()    );
            json.insert("message".to_string(), message.to_json() );
            json.insert("data".to_string(),    data.to_json()    );
            Json::Object(json)
        };
        make_json(self.to_i64(), self.to_message(), self.to_data())
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Error {
    pub fn from_i64(n: i64) -> Result< Error, &'static str> {
        match n {
            -32700i64 => Ok(Error::ParseError),
            -32600i64 => Ok(Error::InvalidRequest),
            -32601i64 => Ok(Error::MethodNotFound),
            -32602i64 => Ok(Error::InvalidParams),
            -32603i64 => Ok(Error::InternalError),
            code@ -32099 ... -32000 => Ok(Error::ServerError(code, "".to_string(), None)),
            code@ _                 => Ok(Error::Unregister(code, "".to_string(), None))
        }
    }
    pub fn to_i64(&self) -> i64 {
        match *self {
            Error::ParseError     => -32700i64,
            Error::InvalidRequest => -32600i64,
            Error::MethodNotFound => -32601i64,
            Error::InvalidParams  => -32602i64,
            Error::InternalError  => -32603i64,
            Error::ServerError(ref code, _, _) => code.clone(),
            Error::Unregister (ref code, _, _) => code.clone()
        }
    }
    pub fn to_message(&self) -> String {
        match *self {
            Error::ParseError     => "Parse error".to_string(),
            Error::InvalidRequest => "Invalid Request".to_string(),
            Error::MethodNotFound => "Method not found".to_string(),
            Error::InvalidParams  => "Invalid method parameter(s)".to_string(),
            Error::InternalError  => "Internal error".to_string(),
            Error::ServerError(_, ref message, _) => message.clone(),
            Error::Unregister (_, ref message, _) => message.clone(),
        }
    }
    pub fn to_data(&self) -> Option<Json> {
        match *self {
            Error::ParseError     => None,
            Error::InvalidRequest => None,
            Error::MethodNotFound => None,
            Error::InvalidParams  => None,
            Error::InternalError  => None,
            Error::ServerError(_, _, ref data) => data.clone(),
            Error::Unregister (_, _, ref data) => data.clone(),
        }
    }
    pub fn set_message(&mut self, msg: String) -> bool {
        match self {
            &mut Error::ServerError(_, ref mut message, _) => {
                *message = msg;
                true
            },
            &mut Error::Unregister(_, ref mut message, _)  => {
                *message = msg;
                true
            },
            _ => false
        }
    }
    pub fn set_data(&mut self, _data: Option<Json>) -> bool {
        match self {
            &mut Error::ServerError(_, _, ref mut data) => {
                *data = _data;
                true
            },
            &mut Error::Unregister(_, _, ref mut data)  => {
                *data = _data;
                true
            },
            _ => false
        }
    }
    pub fn _parse_error(obj: &Object) -> Result<(i64, String, Option<Json>), ()> {
        let result = match obj.get("error") {
            Some(result) => result,
            None         => return Err(())
        };
        if !result.is_object() {
            return Err(());
        }
        match result.as_object() {
            Some(ref result) => {
                let code    = match result.get("code"){
                    Some(code) => {
                        if code.is_i64() {
                            code.as_i64().unwrap()
                        } else if code.is_u64() {
                            code.as_u64().unwrap() as i64
                        } else if code.is_f64() {
                            code.as_f64().unwrap() as i64
                        } else {
                            return Err(());
                        }
                    },
                    None => return Err(())
                };
                let message = match result.get("message"){
                    Some(message) => message.as_string().unwrap().to_string(),
                    None          => String::new()
                };
                let data    = match result.get("data"){
                    Some(data) => match data.is_null(){
                        true  => None,
                        false => Some(data.clone())
                    },
                    None       => None
                };
                // let error = match Error::from_i64(code){
                //     Ok(ref mut e) => {
                //         e.set_message(message);
                //         e.set_data(data);
                //         e
                //     },
                //     Err(_) => return Err(())
                // };
                // Ok(*&*error)
                Ok((code, message, data))

            },
            None => Err(())
        }
    }
}
