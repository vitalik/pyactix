use pyo3::prelude::*;


mod server;
mod routes;
mod shared_socket;
mod pyexec;

use server::Server;
use routes::OperationInfo;
use shared_socket::SocketHeld;



#[pyfunction]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyactix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_class::<OperationInfo>()?;
    m.add_class::<Server>()?;
    m.add_class::<SocketHeld>()?;
    Ok(())
}
