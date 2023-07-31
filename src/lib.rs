use pyo3::prelude::*;

mod pyexec;
mod request;
mod response;
mod routes;
mod server;
mod shared_socket;

use request::Request;
use response::Response;
use routes::OperationInfo;
use server::Server;
use shared_socket::SocketHeld;

#[pyfunction]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyactix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_class::<OperationInfo>()?;
    m.add_class::<Server>()?;
    m.add_class::<SocketHeld>()?;
    m.add_class::<Request>()?;
    m.add_class::<Response>()?;
    Ok(())
}
