//use core::;
use protocol_spec::{core::Server, http::{build_http_request_protocol, build_http_response_protocol, HttpConfig, HttpRequestFactory, HttpResponseFactory}};
use std::vec;

#[tokio::main]
async fn main() {
    let a = 10u32;
    let b = a.to_be_bytes();
    let mut http_server_builder = 
    protocol_spec::core::ServerInstanceBuilder::<HttpConfig>::default();
    http_server_builder = http_server_builder
        .hosts(vec!["127.0.0.1:8080".to_string()/* , "192.168.1.2:8080".to_string() */])
        .request_factory(HttpRequestFactory(build_http_request_protocol()))
        .response_factory(HttpResponseFactory(build_http_response_protocol()));
        
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