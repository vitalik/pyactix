use anyhow::Result;
use pyo3::prelude::*;

use crate::request::Request;
use crate::response::Response;
use crate::routes::OperationInfo;

//type Response = String;

#[inline]
pub async fn execute_operation(operation: &OperationInfo, request: Request) -> PyResult<Response> {
    if operation.is_async {
        let output = Python::with_gil(|py| {
            let function_output = get_function_output(py, operation, request)?;
            pyo3_asyncio::tokio::into_future(function_output)
        })?
        .await?;

        return Python::with_gil(|py| -> PyResult<Response> { output.extract(py) });
    };

    Python::with_gil(|py| -> PyResult<Response> {
        get_function_output(py, operation, request)?.extract()
    })
}

fn get_function_output<'a>(
    py: Python<'a>,
    operation: &'a OperationInfo,
    request: Request,
    // path_params: &'a HashMap<String, String>,
) -> Result<&'a PyAny, PyErr> {
    let handler = operation.handler.as_ref(py);

    let py_request = Py::new(py, request)?;

    handler.call1((py_request,))
}

pub fn get_traceback(error: &PyErr) -> String {
    Python::with_gil(|py| -> String {
        if let Some(traceback) = error.traceback(py) {
            let msg = match traceback.format() {
                Ok(msg) => format!("\n{msg} {error}"),
                Err(e) => e.to_string(),
            };
            return msg;
        };

        error.value(py).to_string()
    })
}
