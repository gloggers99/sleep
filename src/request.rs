#[derive(Debug)]
pub enum MethodType {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

pub struct Method {
    pub method_type: MethodType,
    pub path: String,
    pub version: String
}

impl Method {
    pub fn parse(header: &String) -> Result<Method, &'static str> {
        let parts: Vec<&str> = header.split(' ').collect();
        if parts.len() != 3 {
            return Err("")
        }

        Ok(Method {
            method_type: match parts[0] {
                "GET" => MethodType::GET,
                "HEAD" => MethodType::HEAD,
                "POST" => MethodType::POST,
                "PUT" => MethodType::PUT,
                "DELETE" => MethodType::DELETE,
                "CONNECT" => MethodType::CONNECT,
                "OPTIONS" => MethodType::OPTIONS,
                "TRACE" => MethodType::TRACE,
                "PATCH" => MethodType::PATCH,
                _ => {
                    return Err("sleep: Invalid MethodType from request: {}");
                }
            },
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        })
    }
}

pub struct Request {
    pub method: Method
}

impl Request {
    pub fn parse(request: Vec<String>) -> Result<Request, &'static str> {
        let method: Method = match Method::parse(&request[0]) {
            Ok(method) => method,
            Err(e) => return Err(e)
        };
        
        Ok(Request {
            method
        })
    }
}