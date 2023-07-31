use actix_web::web::Query;
use actix_web::HttpRequest;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::vec::Vec;

#[pyclass]
pub struct Request {
    #[pyo3(get)]
    pub method: String,

    #[pyo3(get)]
    pub scheme: String,

    #[pyo3(get)]
    pub host: String,

    #[pyo3(get)]
    pub path: String,

    #[pyo3(get)]
    pub path_params: HashMap<String, String>,

    #[pyo3(get)]
    pub query_string: String,

    #[pyo3(get)]
    pub query_params: Vec<(String, String)>,

    #[pyo3(get)]
    pub headers: HashMap<String, String>,
    // #[pyo3(get)]
    // pub cookies: String,//HashMap<String, String>,
}

impl Request {
    pub fn from_actix(req: &HttpRequest) -> Self {
        let conn_info = req.connection_info();
        let query_string = req.query_string();

        let query_params = Query::<Vec<(String, String)>>::from_query(req.query_string())
            .unwrap_or(Query(Vec::new()))
            .into_inner();

        let headers = req
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        Self {
            method: req.method().to_string(),
            scheme: conn_info.scheme().to_string(),
            host: conn_info.host().to_string(),
            path: req.uri().path().to_string(),
            path_params: req
                .match_info()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            query_string: query_string.to_string(),
            query_params: query_params,

            //cookies: format!("{:?}", req.cookies()), //get_all_cookies(req), // TODO: does not work - panics
            headers: headers,
        }
    }
}

#[pymethods]
impl Request {
    pub fn hello(&self) -> PyResult<String> {
        Ok(format!(
            "Hello  method: {}, path: {}",
            self.method, self.path
        ))
    }
}
