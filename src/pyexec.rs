use std::sync::Arc;
use std::collections::HashMap;
use anyhow::Result;
use pyo3::prelude::*;
use pyo3_asyncio::TaskLocals;

use crate::routes::{OperationInfo};


type Response = String;

#[inline]
pub async fn execute_operation(
    //request: &Request,
    operation: &OperationInfo,
    path_params: &HashMap<String, String>
) -> PyResult<Response> {
    if operation.is_async {
        let output = Python::with_gil(|py| {
            let function_output = get_function_output(py, operation, path_params)?;
            pyo3_asyncio::tokio::into_future(function_output)
        })?
        .await?;

        return Python::with_gil(|py| -> PyResult<Response> { output.extract(py) });
    };

    Python::with_gil(|py| -> PyResult<Response> {
        get_function_output(py, operation, path_params)?.extract()
    })
}


fn get_function_output<'a>(
    py: Python<'a>,
    operation: &'a OperationInfo,
    path_params: &'a HashMap<String, String>
) -> Result<&'a PyAny, PyErr>
{
    let handler = operation.handler.as_ref(py);

    handler.call1((path_params.to_object(py),))
}
