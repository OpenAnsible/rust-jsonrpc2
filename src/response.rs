
use std::collections::BTreeMap;
use std::string::ToString;
use ::{Json, ToJson, Error};

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
