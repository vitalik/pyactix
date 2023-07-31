use actix_web::http::Method;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fmt;

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

pub type PathOperations = HashMap<Method, OperationInfo>;

pub struct HttpRouter {
    pub operations_paths: HashMap<String, PathOperations>,
    // TODO: ^ some other structure as we need to keep insert order
}

impl HttpRouter {
    pub fn new(operations: Vec<OperationInfo>) -> Self {
        let mut operations_paths: HashMap<String, PathOperations> = HashMap::new();

        for operation in operations {
            let method = method_str_to_actix(&operation.method);
            let path_methods = operations_paths
                .entry(operation.path.clone())
                .or_insert_with(|| PathOperations::new());
            path_methods.insert(method.clone(), operation);
        }

        Self {
            operations_paths: operations_paths,
        }
    }
}

impl fmt::Debug for HttpRouter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("len", &self.operations_paths.len())
            .finish()
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
