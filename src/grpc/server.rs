use tonic::{transport::Server, Request, Response, Code, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // if you want to output only trace_id, change this attribute and trace_fn
    // #[tracing::instrument(skip(self, request))
    #[tracing::instrument]
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status>{
        tracing::info!("Got a request: {:?}", request);
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", some_logic(&request.get_ref().name).await?),
        };
        tracing::info!("Done");
        Ok(Response::new(reply))
    }
}

async fn some_logic(name: &str) -> Result<String, Status> {
    tracing::info!("Run some logic");
    match name {
        "foo" => {
            tracing::error!("Failed some_logic");
            Err(Status::new(Code::InvalidArgument, "who is foo"))
        }
        _ => Ok(name.to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        // if you want to output only trace_id, change trace_fn and callback func's attribute
        // .trace_fn(|header| {
        //     let trace_id = header
        //                      .get("trace_id")
        //                      .map(|value| value.to_str().unwrap_or("unknown"))
        //                      .unwrap_or("Uknown")
        //      tracing::info_span!("gRPC server", trace_id=trace_id);
        // }
        .trace_fn(|_| tracing::info_span!("gRPC server"))
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
