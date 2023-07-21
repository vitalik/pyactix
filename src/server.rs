use pyo3::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use std::thread;
use actix_web::*;

use crate::shared_socket::SocketHeld;
use crate::routes::{OperationInfo, HttpRouter};
use crate::pyexec::execute_operation;



async fn request_handler(req: &HttpRequest, router: web::Data<Arc<HttpRouter>>) -> impl Responder {
    match router.find(req.method(), req.path()) {
        Ok(op_match) => {
            // println!("operation: {:?}", operation);
            let output = execute_operation(&op_match.operation, &op_match.params).await.unwrap_or_else(|e| {

                let traceback = get_traceback(&e);
                println!(
                    "Error while executing route function for endpoint `{}`: {}",
                    req.uri().path(),
                    &traceback
                );

                traceback

                //HttpResponse::InternalServerError().body(traceback)
            });

            HttpResponse::Ok().body(output)
        }, 
        Err(err) => {
            println!("err: {:?}", err);
            HttpResponse::NotFound().body(format!("err: {:?}", err))
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
        Self { router: Arc::new(HttpRouter::new(operations)) }
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

                    let task_locals = task_locals.clone();

                    app = app.app_data(web::Data::new(router.clone()));

                    app.default_service(web::route().to(
                        move |router: web::Data<Arc<HttpRouter>>, req: HttpRequest| {

                            pyo3_asyncio::tokio::scope_local(task_locals.clone(), async move {
                                let start_time = Instant::now();
                                let result = request_handler(&req, router).await;
                                println!("{} {} [{:.6}]", req.method(), req.path(), (Instant::now() - start_time).as_secs_f64());
                                result
                            })
                            
                        })
                    )
                    
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


fn get_traceback(error: &PyErr) -> String {
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

