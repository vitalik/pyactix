use actix_http::KeepAlive;
use actix_web::*;
use pyo3::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use crate::pyexec::{execute_operation, get_traceback};
use crate::request::Request;
use crate::routes::{HttpRouter, OperationInfo, PathOperations};
use crate::shared_socket::SocketHeld;

async fn not_found() -> impl Responder {
    HttpResponse::NotFound()
        .content_type("application/json")
        .body("{\"error\": \"not found\"}")
}

async fn request_handler(req: &HttpRequest, path_methods: &PathOperations) -> impl Responder {
    let operation = path_methods.get(&req.method());

    if operation.is_none() {
        return format!("method not allowed: {:?}", req.method())
            .into_bytes()
            .customize()
            .insert_header(("content-type", "text/plain"))
            .with_status(http::StatusCode::METHOD_NOT_ALLOWED);
    }

    let operation = operation.unwrap();

    let request = Request::from_actix(req);

    let result = execute_operation(&operation, request).await;

    match result {
        Ok(output) => output
            .content
            .customize()
            .insert_header(("content-type", "text/plain"))
            .with_status(http::StatusCode::from_u16(output.status_code).unwrap()),
        Err(e) => {
            let traceback = get_traceback(&e);
            println!("Error at `{}`:\n{}", req.uri().path(), &traceback);

            traceback
                .into_bytes()
                .customize()
                .insert_header(("content-type", "text/plain"))
                .with_status(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[pyclass]
pub struct Server {
    router: Arc<HttpRouter>,
}

#[pymethods]
impl Server {
    #[new]
    pub fn new(operations: Vec<OperationInfo>) -> Self {
        Self {
            router: Arc::new(HttpRouter::new(operations)),
        }
    }

    pub fn start(
        &mut self,
        py: Python,
        socket: &PyCell<SocketHeld>,
        workers: usize,
    ) -> PyResult<()> {
        println!("start: {:?}", socket);
        println!("worker: {:?}", workers);

        let raw_socket = socket.try_borrow_mut()?.get_socket();

        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("new_event_loop")?;
        asyncio.call_method1("set_event_loop", (event_loop,))?;

        let task_locals = pyo3_asyncio::TaskLocals::new(event_loop).copy_context(py)?;

        let router = self.router.clone();

        thread::spawn(move || {
            actix_web::rt::System::new().block_on(async move {
                HttpServer::new(move || {
                    let mut app = App::new();

                    for path in router.operations_paths.keys() {
                        let task_locals = task_locals.clone();
                        let path_methods = router.operations_paths.get(path).unwrap().clone();

                        app =
                            app.service(web::resource(path.clone()).to(move |req: HttpRequest| {
                                let path_methods = path_methods.clone(); // TODO: how to avoid clone?
                                pyo3_asyncio::tokio::scope_local(task_locals.clone(), async move {
                                    let start_time = Instant::now();
                                    let result = request_handler(&req, &path_methods).await;
                                    println!(
                                        "{} {} [{:.6}]",
                                        req.method(),
                                        req.path(),
                                        (Instant::now() - start_time).as_secs_f64()
                                    );
                                    result
                                })
                            }));
                    }

                    app = app.default_service(web::route().to(not_found));

                    app
                })
                //.keep_alive(KeepAlive::Os)
                .workers(workers)
                .client_request_timeout(std::time::Duration::from_secs(0))
                .listen(raw_socket.try_into().unwrap())
                .unwrap()
                .run()
                .await
                .unwrap();
            });
        });

        println!("Ready. Ctrl+C to stop.");
        let event_loop = (*event_loop).call_method0("run_forever");
        if event_loop.is_err() {
            println!("\nCtrl c handler");
            // Python::with_gil(|py| {
            //     pyo3_asyncio::tokio::run(py, async move {
            //         execute_event_handler(shutdown_handler, &task_locals.clone())
            //             .await
            //             .unwrap();
            //         Ok(())
            //     })
            // })?;
            println!("Done");
            //abort();
        }

        Ok(())
    }
}
