#[warn(non_shorthand_field_patterns)]
use std::collections::BTreeMap;
// use std::str::FromStr;
use std::string::ToString;
use ::{Json, ToJson};

#[derive(Debug, Clone)]
struct RpcError {
    code   : i64,
    message: &'static str,
    data   : Option<Json>
}

#[derive(Debug, Clone)]
pub enum Error {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerError{ code: i64, message: &'static str, data: Option<Json> } // -32000 to -32099
}

impl ToJson for RpcError {
    fn to_json (&self) -> Json {
        let mut json = BTreeMap::new();
        json.insert("code".to_string(),    self.code.to_json()    );
        json.insert("message".to_string(), self.message.to_json() );
        json.insert("data".to_string(),    self.data.to_json()    );
        Json::Object(json)
    }
}

impl ToJson for Error {
    fn to_json(&self) -> Json {
        match *self {
            Error::ParseError     => RpcError{ code: self.to_i64(), message: self.to_message(), data: None }.to_json(),
            Error::InvalidRequest => RpcError{ code: self.to_i64(), message: self.to_message(), data: None }.to_json(),
            Error::MethodNotFound => RpcError{ code: self.to_i64(), message: self.to_message(), data: None }.to_json(),
            Error::InvalidParams  => RpcError{ code: self.to_i64(), message: self.to_message(), data: None }.to_json(),
            Error::InternalError  => RpcError{ code: self.to_i64(), message: self.to_message(), data: None }.to_json(),
            Error::ServerError{ 
                data: ref data, .. }  => RpcError{ code: self.to_i64(), message: self.to_message(), data: data.clone() }.to_json(),
        }
    }
}

impl ToString for RpcError {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}
impl ToString for Error {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Error {
    pub fn from_i64(n: i64) -> Result<Error, &'static str> {
        match n {
            -32700i64 => Ok(Error::ParseError),
            -32600i64 => Ok(Error::InvalidRequest),
            -32601i64 => Ok(Error::MethodNotFound),
            -32602i64 => Ok(Error::InvalidParams),
            -32603i64 => Ok(Error::InternalError),
            code@ -32099 ... -32000 => Ok(Error::ServerError {code: code, message: "", data: None}),
            _         => Err("Invalid RPC Error Code.")
        }
    }
    pub fn to_i64(&self) -> i64 {
        match *self {
            Error::ParseError     => -32700i64,
            Error::InvalidRequest => -32600i64,
            Error::MethodNotFound => -32601i64,
            Error::InvalidParams  => -32602i64,
            Error::InternalError  => -32603i64,
            Error::ServerError{
                code: code, ..}   => code
        }
    }
    pub fn to_message(&self) -> &'static str {
        match *self {
            Error::ParseError     => "Parse error",
            Error::InvalidRequest => "Invalid Request",
            Error::MethodNotFound => "Method not found",
            Error::InvalidParams  => "Invalid method parameter(s)",
            Error::InternalError  => "Internal error",
            Error::ServerError{
                message: message, ..} => message
        }
    }
}
