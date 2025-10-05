//use core::;
use protocol_spec::{core::Server, http::{build_http_request_protocol, build_http_response_protocol, HttpConfig, HttpRequestFactory, HttpResponseFactory}};
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::vec;

#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
        .init();
    let a = 10u32;
    let b = a.to_be_bytes();
    let mut http_server_builder =   
    protocol_spec::core::ServerInstanceBuilder::<HttpConfig>::default();
    http_server_builder = http_server_builder
        .hosts(vec!["127.0.0.1:8080".to_string()/* , "192.168.1.2:8080".to_string() */])
        .request_factory(HttpRequestFactory::new(build_http_request_protocol()))
        .response_factory(HttpResponseFactory::new(build_http_response_protocol()));
        
    let server = http_server_builder.build().unwrap();

    let server_instance = Box::leak(Box::new(server));
    
    server_instance.start()
        .await
        .unwrap();
    print!("Server started...");
    loop{
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}