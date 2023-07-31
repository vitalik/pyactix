use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyBytes, PyDict, PyString},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Response {
    #[pyo3(get, set)]
    pub status_code: u16,

    #[pyo3(get, set)]
    pub headers: Py<PyDict>,

    #[pyo3(get)]
    //pub content: Py<PyAny>,
    pub content: Vec<u8>,
}

#[pymethods]
impl Response {
    #[new]
    pub fn new(py: Python, content: Py<PyAny>) -> PyResult<Self> {
        let content_bytes = get_body_from_pyobject(content.as_ref(py))?;

        Ok(Self {
            status_code: 200,
            headers: PyDict::new(py).into_py(py),
            content: content_bytes,
        })
    }
}

pub fn get_body_from_pyobject(body: &PyAny) -> PyResult<Vec<u8>> {
    if let Ok(s) = body.downcast::<PyString>() {
        Ok(s.to_string().into_bytes())
    } else if let Ok(b) = body.downcast::<PyBytes>() {
        Ok(b.as_bytes().to_vec())
    } else {
        Err(PyValueError::new_err(
            "Could not convert specified body to bytes",
        ))
    }
}
