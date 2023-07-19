use std::collections::HashMap;
use pyo3::prelude::*;
use matchit::Router as MatchItRouter;
use actix_web::http::Method;

#[pyclass]
#[derive(Debug, Clone)]
pub struct OperationInfo {
    #[pyo3(get, set)]
    pub method: String,
    #[pyo3(get, set)]
    pub path: String,
    #[pyo3(get, set)]
    pub handler: Py<PyAny>,
    #[pyo3(get, set)]
    pub is_async: bool,
}

#[pymethods]
impl OperationInfo {
    #[new]
    pub fn new(method: &str, path: &str, handler: Py<PyAny>, is_async: bool) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            handler,
            is_async,
        }
    }
}


type RouteMap = MatchItRouter<OperationInfo>; // RwLock<>?

pub struct HttpRouter {
    method_routes: HashMap<Method, RouteMap>,
}

#[derive(Debug)]
pub struct MatchedOperation {
    pub operation: OperationInfo,
    pub params: HashMap<String, String>,
}

impl HttpRouter {

    // TODO: actually better find path first 
    // and then method - and if path exist and method not - return 405

    pub fn new(operations: Vec<OperationInfo>) -> Self {

        let mut method_routes = HashMap::new();

        for operation in operations {
            let method = method_str_to_actix(&operation.method);
            let router = method_routes.entry(method).or_insert_with(|| MatchItRouter::new());
            router.insert(&operation.path, operation.clone()).unwrap();
        }

        Self {
            method_routes: method_routes,
        }
    }


    pub fn find(&self, method: &Method, path: &str) -> Result<MatchedOperation, String> {
        let router = self.method_routes.get(method);

        if router.is_none() {
            return Err(format!("No such method"));
        }


        match router.unwrap().at(path) {
            Ok(val) => {
                let mut path_params = HashMap::new();
                for (key, value) in val.params.iter() {
                    path_params.insert(key.to_string(), value.to_string());
                }
                Ok(MatchedOperation {
                    operation: val.value.clone(), 
                    params: path_params
                })
            },
            Err(e) => Err(format!("Not found: {}", e)),
        }


    }
    
}

fn method_str_to_actix(method: &str) -> Method {
    match method {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        "CONNECT" => Method::CONNECT,
        "PATCH" => Method::PATCH,
        "TRACE" => Method::TRACE,
        _ => panic!("Unsupported HTTP method: {}", method),
    }
}