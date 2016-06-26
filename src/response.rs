
use std::collections::BTreeMap;
use std::str::FromStr;
use std::string::ToString;
use ::{Json, ToJson, Object, Error, Request};

#[derive(Debug, Clone)]
pub enum Response {
    Success{jsonrpc: String, result: Option<Json>, id: Option<i64>},
    Error  {jsonrpc: String, error : Error,        id: Option<i64>}
}

impl ToJson for Response {
    fn to_json(&self) -> Json {
        match *self {
            Response::Success{
                jsonrpc: ref jsonrpc,
                result : ref result,
                id     : ref id } => {
                    let mut json = BTreeMap::new();
                    json.insert("jsonrpc".to_string(), jsonrpc.to_json() );
                    json.insert("result".to_string(),  result.to_json()  );
                    json.insert("id".to_string(),      id.to_json()      );
                    Json::Object(json)
            },
            Response::Error{
                jsonrpc: ref jsonrpc,
                error  : ref error,
                id     : ref id } => {
                    let mut json = BTreeMap::new();
                    json.insert("jsonrpc".to_string(), jsonrpc.to_json() );
                    json.insert("error".to_string(),   error.to_json()   );
                    json.insert("id".to_string(),      id.to_json()      );
                    Json::Object(json)
            }
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl FromStr for Response {
    type Err = Error;
    fn from_str(s: &str) -> Result<Response, Error> {
        match Json::from_str(s) {
            Ok(json) => {
                match Response::parse(json) {
                    Ok(response) => Ok(response),
                    Err(err)    => Err(err)
                }
            },
            Err(_)  => Err(Error::ParseError)
        }
    }
}

impl Response {
    pub fn parse(j: Json) -> Result<Response, Error> {
        if !j.is_object() {
            return Err(Error::ParseError);
        }
        let obj = j.as_object().unwrap();

        let version = Request::_parse_version(&obj);
        let id      = Request::_parse_id(&obj);

        let result  = Response::_parse_result(&obj);
        let error   = Error::_parse_error(&obj);
        

        if version.is_err() && id.is_err() {
            return Err(Error::InternalError);
        }

        if result.is_err() && error.is_err() {
            return Err(Error::InternalError);
        }
        if result.is_err() && error.is_ok() {
            let (code, message, data) = error.ok().unwrap();
            let mut error = Error::from_i64(code).unwrap();
            error.set_message(message);
            error.set_data(data);
            
            return Ok(Response::Error{
                jsonrpc: version.ok().unwrap(),
                error  : error,
                id     : id.ok().unwrap()
            })
        }
        if result.is_ok() && error.is_err() {
            return Ok(Response::Success{
                jsonrpc: version.ok().unwrap(),
                result : result.ok().unwrap(),
                id     : id.ok().unwrap()
            })
        } else {
            return Err(Error::InternalError);
        }
        
    }
    pub fn _parse_result(obj: &Object) -> Result<Option<Json>, ()> {
        match obj.get("result") {
            Some(result) => {
                if result.is_array() || result.is_object() {
                    Ok(Some(result.clone()))
                } else if result.is_i64() 
                    || result.is_u64() 
                    || result.is_f64()
                    || result.is_boolean()
                    || result.is_string() {
                        Ok(Some(result.clone()))
                } else if result.is_null() {
                    Ok(None)
                } else {
                    Err(())
                }
            },
            None => Err(())
        }
    }

}