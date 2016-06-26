
use std::collections::BTreeMap;
use std::str::FromStr;
use std::string::ToString;
use ::{Json, ToJson, Object, Error};

#[derive(Debug, Clone)]
pub struct Request {
    jsonrpc: String,
    method : String,
    params : Option<Json>,
    id     : Option<i64>
}

impl FromStr for Request {
    type Err = Error;
    fn from_str(s: &str) -> Result<Request, Error> {
        match Json::from_str(s) {
            Ok(json) => {
                match Request::parse(json) {
                    Ok(request) => Ok(request),
                    Err(err)    => Err(err)
                }
            },
            Err(_)  => Err(Error::ParseError)
        }
    }
}

impl ToJson for Request {
    fn to_json(&self) -> Json {
        let mut json = BTreeMap::new();
        json.insert("jsonrpc".to_string(), self.jsonrpc.to_json() );
        json.insert("method".to_string(),  self.method.to_json()  );
        json.insert("params".to_string(),  self.params.to_json()  );
        json.insert("id".to_string(),      self.id.to_json()      );
        Json::Object(json)
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Request {
    pub fn parse(j: Json) -> Result<Request, Error> {
        if !j.is_object() {
            return Err(Error::ParseError);
        }

        let obj = j.as_object().unwrap();

        let version = Request::_parse_version(&obj);
        let method  = Request::_parse_method(&obj);
        let params  = Request::_parse_params(&obj);
        let id      = Request::_parse_id(&obj);

        if version.is_err() && id.is_err() {
            return Err(Error::InvalidRequest);
        }
        if method.is_err() {
            return Err(Error::MethodNotFound);
        }
        if params.is_err() {
            return Err(Error::InvalidParams);
        }
        Ok(Request {
            jsonrpc: version.ok().unwrap(),
            method : method.ok().unwrap(),
            params : params.ok().unwrap(),
            id     : id.ok().unwrap()
        })
    }
    fn _parse_version (obj: &Object) -> Result<String, ()> {
        match obj.get("jsonrpc") {
            Some(version) => {
                match version.as_string().unwrap() {
                    // JsonRpc Version Must Be 2.0. 
                    "2.0" | "2" => Ok("2.0".to_string()),
                    _           => Err(())
                }
            },
            None => Err(())
        }
    }
    fn _parse_method (obj: &Object) -> Result<String, ()> {
        match obj.get("method") {
            Some(method) => {
                match method.is_string() {
                    true  => Ok(method.as_string().unwrap().to_string()),
                    false => Err(())
                }
            },
            None => Err(())
        }
    }
    fn _parse_params (obj: &Object) -> Result<Option<Json>, ()> {
        match obj.get("params") {
            Some(params) => {
                if params.is_array() || params.is_object() {
                    Ok(Some(params.clone()))
                } else if params.is_null() {
                    Ok(None)
                } else {
                    Err(())
                }
            },
            None => Err(())
        }
    }
    fn _parse_id (obj: &Object) -> Result<Option<i64>, ()> {
        match obj.get("id") {
            Some(id) => {
                // i.is_number() || i.is_u64()
                if id.is_i64() || id.is_u64() {
                    Ok(Some(id.as_i64().unwrap()))
                } else if id.is_null() {
                    Ok(None)
                } else {
                    Err(())
                }
            },
            None => Err(())
        }
    }

    pub fn jsonrpc(&self) -> String {
        self.jsonrpc.clone()
    }
    pub fn method(&self) -> String {
        self.method.clone()
    }
    pub fn params(&self) -> Option<Json> {
        self.params.clone()
    }
    pub fn id(&self) -> Option<i64> {
        self.id.clone()
    }
}

