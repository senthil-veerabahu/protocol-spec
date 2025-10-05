#![allow(unused_variables)]
#![allow(unused_mut)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::to_string;


use crate::core::*;
use crate::mapping_extractor::{traverse_spec, DefaultMapper, SpecTraverse, ToSpecType};


use paste::paste;
use std::collections::HashMap;
use std::io::Error;
use std::str::{self, from_utf8, Utf8Error};
use crate::core::SpecName::*;

type Vecu8 = Vec<u8>;

#[macro_export]
macro_rules! define_header_getter_setters {
    ($(($header:expr, $method_suffix:expr, $type:ident)),*) => {

        $(
            paste! {
                fn [ <set_ $method_suffix >](&mut self, value: $type);
    
                fn [ <get_ $method_suffix >](&self,) ->Option<get_return_type!($type)>;

            }
        )*
    }
}

#[macro_export]
macro_rules! create_header_getter_setters {
        ( $(($header:expr, $method_suffix:expr, $type:ident)),* ) => {

        $(
            paste! {
                fn [ <set_ $method_suffix >](&mut self, value: $type) {
                    self.add_info($header.to_string(), create_value_type!($type, value));
                }

                create_header_getters!(
                    ($header, $method_suffix, $type)
                );
            }
        )*
    };
}

#[macro_export]
macro_rules! create_with_headers {
    (($header:expr, $method_suffix:expr, $type:ident)) => {
        paste! {
            #[allow(unused)]
            fn [ < with_ $method_suffix > ](&mut self, value: $type) ->&mut Self {
                self.0
                    .add_info($header.to_string(), create_value_type!($type, value));
                self

            }
        }
    };
}

#[macro_export]
macro_rules! create_value_type {
    (String, $value:expr) => {
        Value::String($value)
    };

    (i64, $value:expr) => {
        Value::SignedNumber64($value)
    };

    (u64, $value:expr) => {
        Value::UnSignedNumber64($value)
    };

    (u32, $value:expr) => {
        Value::UnSignedNumber32($value)
    };

    (i16, $value:expr) => {
        Value::SignedNumber16($value)
    };

    (u16, $value:expr) => {
        Value::UnSignedNumber16($value)
    };

    

   (Vecu8,  $value:expr) =>{
        Value::U8Vec($value)
   }
}

#[macro_export]
macro_rules! get_underlying_type {

    (String, $value:expr) => {
        $value.get_string_value()
    };

    (i64, $value:expr) => {
        $value.get_signed_num_64_value()
    };

    (u64, $value:expr) => {
        $value.get_unsigned_num_64_value()
    };

    (u32, $value:expr) => {
        $value.get_unsigned_num_32_value()
    };

    (i16, $value:expr) => {
        $value.get_signed_num_16_value()
    };

    (u16, $value:expr) => {
        $value.get_unsigned_num_16_value()
    };

    (usize, $value:expr) => {
        $value.get_unsigned_num_value()
    };

   (Vecu8,  $value:expr) =>{
        $value.get_u8_vec()
   }
}

#[macro_export]
macro_rules! get_return_type {
    (Vecu8) => {
        &Vec<u8>
    };

    ($type:ident) => {
        $type
    };
}

#[macro_export]
macro_rules! create_header_getters {
    ($(($header:expr, $method_suffix:expr,$type:ident)),*) => {

        $(
            paste! {
                fn [ <get_ $method_suffix >](&self,) -> Option<get_return_type!($type)> {
                    match self.get_info(&$header.to_string()){
                        Some(value_type) =>{
                            get_underlying_type!($type, value_type)
                        },
                        None => None
                    }
                    
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! impl_request_info {
    ($(($header:expr, $method_suffix:expr, $type:ident)),*) => {

        #[allow(unused)]
        trait HttpReqInfoHeaderProvider: InfoProvider {
            //Request Headers
            $(
                define_header_getter_setters!(($header, $method_suffix, $type));                                
            )*

        }

        impl HttpReqInfoHeaderProvider for HttpRequestInfo {
            $(
                create_header_getter_setters!( ($header, $method_suffix, $type) );
            )*
        }
    }
}

#[macro_export]
macro_rules! impl_request_info_and_builder {
    ($(($header:expr, $method_suffix:expr, $type:ident )),* ) => {

        impl_request_info!( $( ($header, $method_suffix, $type) ),* );

        impl HttpRequestBuilder {

            #[allow(unused)]
            fn new() -> Self {
                HttpRequestBuilder(HttpRequestInfo::default())
            }

            $(
                create_with_headers!(($header, $method_suffix, $type));
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_response_info {
    ($(($header:expr, $method_suffix:expr, $type:ident)),*) => {

        #[allow(unused)]
        trait HttpResInfoHeaderProvider: InfoProvider {
            //Response Headers
            $(
                define_header_getter_setters!(
                    ($header, $method_suffix, $type)
                );
            )*
        }

        impl HttpResInfoHeaderProvider for HttpResponseInfo {

            $(
            create_header_getter_setters!(($header, $method_suffix, $type));
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_response_info_and_builder {
    ($(($header:expr, $method_suffix:expr, $type:ident)),* ) => {

        impl_response_info!( $( ($header, $method_suffix, $type) ),* );

        impl <'a> HttpResponseBuilder<'a> {

            fn build(&mut self)-> HttpResponseInfo{
                std::mem::take(&mut self.0)
            }

            $(
                create_with_headers!(($header, $method_suffix, $type));
            )*
        }
    };
}

pub struct HttpRequestInfo {
    request_type: Value,
    headers: HashMap<String, Value>,
    protocol_version: Option<Value>,
    request_uri: Option<Value>,
    request_body: Option<Value>,
    mapper: Box<dyn Mapper>,
}

impl Default for HttpRequestInfo {
    fn default() -> Self {
        HttpRequestInfo {
            request_type: Value::String("GET".to_string()),
            headers: Default::default(),
            protocol_version: None,
            request_uri: None,
            request_body: Default::default(),
            mapper: Box::new(DefaultMapper::new()),
        }
    }
}

impl RequestInfo for HttpRequestInfo {

    

}
struct HttpRequestBuilder(HttpRequestInfo);

pub struct HttpResponseInfo {
    status_code: Value,
    status_text: Value,
    headers: HashMap<String, Value>,
    protocol_version: Option<Value>,
    response_body: Option<Value>,
    mapper: Box<dyn Mapper>,
}


impl ResponseInfo for HttpResponseInfo {
    fn add_defaults(&mut self) {
        /*status_code: Value::String("200".to_string()),
            headers: HashMap::new(),
            protocol_version: Some(Value::String("HTTP/1.1".to_string())),
            status_text: Value::String("OK".to_string()),
            response_body: Default::default(),*/
        let now: DateTime<Utc> = Utc::now();
        self.add_info("status_code".to_owned(), Value::String("200".to_string()));
        self.add_info("protocol_version".to_owned(), Value::String("HTTP/1.1".to_string()));
        self.add_info("status_text".to_owned(), Value::String("OK".to_string()));
        self.add_info("Date".to_owned(), Value::String(now.format("%a %d %b %Y %H %M %S GMT").to_string()));
    }
}

use chrono::{DateTime, Utc};
impl Default for HttpResponseInfo {
    fn default() -> Self {
        
        
        let mut response = HttpResponseInfo {
            status_code: Value::String("200".to_string()),
            headers: HashMap::new(),
            protocol_version: Some(Value::String("HTTP/1.1".to_string())),
            status_text: Value::String("OK".to_string()),
            response_body: Default::default(),
            mapper: Box::new(DefaultMapper::new()),
        };
        response
    }
}

struct HttpResponseBuilder<'a>(&'a mut HttpResponseInfo);

impl_request_info_and_builder!(
    ("request_method", request_method, String),
    ("request_uri", request_uri, String),
    ("protocol_version", protocol_version, String),
    ("request_body", request_body, Vecu8),
    ("Content-Length", content_length, String),
    ("Content-Type", content_type, String),
    ("Host", host, String),
    ("Transfer-Encoding", transfer_encoding, String),
    ("User-Agent", user_agent, String),
    ("Accept", accept, String),
    ("Accept-Encoding", accept_encoding, String),
    ("Accept-Language", accept_language, String),
    ("Connection", connection, String),
    ("Cookie", cookie, String),
    ("Authorization", authorization, String)
);

 impl_response_info_and_builder!(
    ("status_code", status_code, String),
    ("status_text", status_text, String),
    ("protocol_version", protocol_version, String),
    ("response_body", response_body, Vecu8),
    ("Content-Length", content_length, String),
    ("Content-Encoding", content_encoding, String),
    ("Content-Type", content_type, String),
    ("Date", date, String),
    ("Etag", etag, String),
    ("Keep-Alive", keep_alive, String),
    ("Last-Modified", last_modified, String),
    ("Location", location, String),
    ("Server", server, String),
    ("Set-Cookie", set_cookie, String),
    ("Transfer-Encoding", transfer_encoding, String),
    ("Vary", vary, String),
    ("WWW-Authenticate", www_authenticate, String),
    ("X-Powered-By", x_powered_by, String),
    ("X-Frame-Options", x_frame_options, String),
    ("X-XSS-Protection", x_xss_protection, String),
    ("X-Content-Type-Options", x_content_type_options, String),
    ("X-Backend-Server", x_backend_server, String),
    ("X-Cache", x_cache, String),
    ("X-Content-Duration", x_content_duration, String),
    ("X-Content-Security-Policy", x_content_security_policy, String),
    ( "X-Content-Security-Policy-Report-Only",  x_content_security_policy_report_only, String),
    ("X-DNS-Prefetch-Control", x_dns_prefetch_control, String),
    ("X-Download-Options", x_download_options, String),
    ( "X-Permitted-Cross-Domain-Policies", x_permitted_cross_domain_policies, String)
); 


pub struct HttpRequestFactory{
    spec:Box<dyn ProtocolSpec>,
}

/* impl Clone for Box<dyn Mapper>{
    fn clone(&self) -> Self {
        self.clone()
    }
} */


impl HttpRequestFactory{
    pub fn new(spec: ListSpec) ->Self{
        
        Self { 
            spec: Box::new(spec),
     
        }
    }
}
pub struct HttpRequestHandler;

#[async_trait]
impl RequestHandler<HttpRequestInfo, HttpResponseInfo> for HttpRequestHandler {
    async fn handle_request(
        &self,
        request: &HttpRequestInfo,
        response: &mut HttpResponseInfo,
    ) -> Result<HttpResponseInfo, ParserError> {
        let method = request.get_request_method().unwrap_or("GET".to_owned());

        if method == "GET" {
             self.handle_get(request, response).await
        } else if method == "POST" {
             self.handle_post(request, response).await
        }else if method == "PUT" {
             self.handle_put(request, response).await
        }else{
            self.handle_delete(request, response).await
        }
    }
}



#[allow(unused)]
#[derive(Serialize, Deserialize)]
struct Product{
    id: u16,
    name: String
}

#[allow(unused)]
impl Product{
    fn new(id: u16, name: String)->Self{
        Product { id, name }
    }
}

impl HttpRequestHandler{
    
    async fn handle_get(&self,
        request: &HttpRequestInfo,
        response: &mut HttpResponseInfo,
        ) -> Result<HttpResponseInfo, ParserError> {
            let get_request_uri = request.get_request_uri().unwrap().to_owned();
            if get_request_uri == "/product/1" {
                let content = to_string(&Product::new(1, "Table".to_owned()))?.into_bytes();
                let response = HttpResponseBuilder(response)
                    .with_status_code("200".to_owned())
                    .with_status_text("OK".to_owned())
                    .with_protocol_version("HTTP/1.1".to_owned())
                    .with_content_type("application/json".to_owned())
                    .with_content_length(content.len().to_string())
                    .with_response_body(content)
                    .build();
                return Ok(response);
            }
           Ok(Default::default())

    }

    async fn handle_post(&self,
        request: &HttpRequestInfo,
        response: &mut HttpResponseInfo,) -> Result<HttpResponseInfo, ParserError> {
            
            if request.get_request_uri().unwrap().as_str() == "/product/1" {
                let body = request.get_request_body().unwrap();
                let content = from_utf8(body)?;
                if request.get_content_type().unwrap_or("application/json".to_owned()) == "application/json" {
                    let product: Product = serde_json::from_str(content).unwrap();
                    let response = HttpResponseBuilder(response)
                    .with_status_code("201".to_owned())
                    .with_status_text("Created".to_owned())                    
                    .with_protocol_version("HTTP/1.1".to_owned())
                    .with_content_length("0".to_owned())
                    .build();
                    return Ok(response);
                }
                
            }
           Ok(Default::default())

    }

    async fn handle_put(&self,
        request: &HttpRequestInfo,
        response: &mut HttpResponseInfo,) -> Result<HttpResponseInfo, ParserError> {
            self.handle_post(request, response).await

    }

    async fn handle_delete(&self,
        request: &HttpRequestInfo,
        response: &mut HttpResponseInfo,) -> Result<HttpResponseInfo, ParserError> {
            if request.get_request_uri().unwrap().as_str() == "/product/1" {
                let body = request.get_request_body().unwrap();
                let content = from_utf8(body)?;
                if request.get_content_type().unwrap_or("application/json".to_owned()) == "application/json" {
                    let product: Product = serde_json::from_str(content).unwrap();
                    let response = HttpResponseBuilder(response)
                    .with_status_code("200".to_owned())
                    .with_status_text("OK".to_owned())                    
                    .with_protocol_version("HTTP/1.1".to_owned())
                    .build();
                    return Ok(response);
                }
                
            }
           Ok(Default::default())

    }
}

impl RequestErrorHandler<HttpRequestInfo, HttpResponseInfo> for HttpRequestHandler {
    fn handle_request_error<E>(
        &self,
        request: &HttpRequestInfo,
        error: E,
    ) -> Result<HttpResponseInfo, ParserError> {
        todo!()
    }
}

impl
    RequestFactory<
        HttpRequestInfo,
        DefaultSerializer,
        HttpRequestHandler,
        HttpRequestHandler,
        HttpResponseInfo,
    > for HttpRequestFactory
{
    fn get_request_spec(&self) -> &Box<dyn ProtocolSpec> {
        &self.spec
    }

    fn create_request_info(&self) -> HttpRequestInfo {
        HttpRequestInfo::default()        
    }

    fn create_request_serializer(&self) -> DefaultSerializer {
        DefaultSerializer {}
    }

    fn create_request_handler(&self) -> HttpRequestHandler {
        HttpRequestHandler{}
    }

    fn create_error_request_handler(&self) -> HttpRequestHandler {
        HttpRequestHandler{}
    }
}

pub struct HttpResponseFactory{
    response_spec: Box<dyn ProtocolSpec>,
}


impl HttpResponseFactory{
    pub fn new(spec: ListSpec) ->Self{
        Self { 
            response_spec: Box::new(spec), 
        }
    }
}

pub struct HttpResponseHandler;

#[async_trait]
impl ResponseHandler<HttpResponseInfo> for HttpResponseHandler {
    fn handle_response(&self, response: &HttpResponseInfo) -> Result<(), ParserError> {
        Ok(())
    }
}

impl ResponseErrorHandler<HttpResponseInfo> for HttpResponseHandler {
    fn handle_response_error<E>(
        &self,
        response_info: &HttpResponseInfo,
        error: E,
    ) -> Result<(), ParserError> {
        Ok(())
    }
}

impl ResponseFactory<HttpResponseInfo, DefaultSerializer, HttpResponseHandler, HttpResponseHandler>
    for HttpResponseFactory
{
    fn get_response_spec(&self) -> &Box<dyn ProtocolSpec> {
        &self.response_spec
    }

    fn create_response_info(&self) -> HttpResponseInfo {
        HttpResponseInfo::default()
    }

    fn create_response_serializer(&self) -> DefaultSerializer {
        DefaultSerializer {}
    }

    fn create_response_handler(&self) -> HttpResponseHandler {
        HttpResponseHandler
    }

    fn create_error_response_handler(&self) -> HttpResponseHandler {
        HttpResponseHandler
    }
}

#[allow(unused)]
pub struct HttpConfig;

impl ProtocolConfig for HttpConfig {
    type REQF = HttpRequestFactory;

    type RESF = HttpResponseFactory;

    type REQI = HttpRequestInfo;

    type RESI = HttpResponseInfo;

    type REQSER = DefaultSerializer;

    type RESSER = DefaultSerializer;

    type REQH = HttpRequestHandler;

    type RESH = HttpResponseHandler;

    type REQERRH = HttpRequestHandler;

    type RESERRH = HttpResponseHandler;
}

impl InfoProvider for HttpResponseInfo {
    fn get_info(&self, key: &String) -> Option<&Value> {
        let key_ref = key.as_str();
        match key_ref {
            "status_code" | "protocol_version" | "status_text" | "response_body"=> {
                return self.get_mapper().get_value_by_key(key_ref);
            }
            _ => {
                return self.mapper.get_value_from_key_value_list(key_ref.to_owned(), "header_name");
            }
        }
    }

    fn add_info(&mut self, key: String, value: Value) {
        match key.as_str() {
            "status_text" | "protocol_version"|"status_code"|"response_body"   => {
                self.get_mapper_mut().add_simple_data(key, value);
            }
            _ => {
                //self.headers.insert(key.to_string(), value);
                self.get_mapper_mut().add_to_key_value_list(key, value, "header_name".to_string(), "header_value".to_string());
            }
        }
    }
    
    fn get_mapper_mut(&mut self) ->&mut Box<dyn Mapper> {
        &mut self.mapper
    }
    
    fn get_mapper(&self) ->&Box<dyn Mapper> {
        &self.mapper
    }
}

impl InfoProvider for HttpRequestInfo {
    fn get_info(&self, key: &String) -> Option<&Value> {
        let key_ref = key.as_str();
        match key_ref {
            "request_method" | "protocol_version" | "request_uri" | "request_body" => {
                //self.mapper.add_mapping_template("proto_name".to_owned(), "spec_name".to_owned());
                return self.get_mapper().get_value_by_key(key_ref);
            }
            _ => {
                return self.get_mapper().get_value_from_key_value_list(key.clone(),"header_name").clone();
            }
        }
    }

    fn add_info(&mut self, key: String, value: Value) {
        match key.as_str() {
            "request_method" | "protocol_version" | "request_uri" | "request_body" => {
                self.get_mapper_mut().add_simple_data(key, value);
            }
            _ => {
                //self.headers.insert(key.to_string(), value);
                self.get_mapper_mut().add_to_key_value_list(key, value, "header_name".to_string(), "header-value".to_string());
            }
        }
    }

    
    
    fn get_mapper_mut(&mut self) ->&mut Box<dyn Mapper> {
        &mut self.mapper
    }
    
    fn get_mapper(&self) ->&Box<dyn Mapper> {
        &self.mapper
    }
}

impl HttpRequestInfo {
    #![allow(unused)]
    fn set_request_type(&mut self, request_type: String) {
        self.request_type = Value::String(request_type);
    }
}

impl From<std::io::Error> for ParserError {
    fn from(error: Error) -> Self {
        ParserError::IOError { error }
    }
}



impl From<Utf8Error> for ParserError {
    fn from(error: Utf8Error) -> Self {
        ParserError::Utf8Error( error )
    }
}

impl From<serde_json::Error> for ParserError {
    fn from(error: serde_json::Error) -> Self {
        ParserError::SerdeError(  error.to_string() )
    }
}


#[allow(unused)]
pub fn build_http_request_protocol() -> ListSpec {
    
    let space = " ";
    let newline = "\r\n";
    let mut spec_builder = ProtoSpecBuilderData::<BuildFromScratch>::new();

    let request_line_placeholder= ProtoSpecBuilderData::<BuildFromScratch>::new_with(Transient("request_line".to_string()), false);
    //let request_line_placeholder = ;

        let request_line_placeholder = 
        request_line_placeholder.inline_value_follows(Name("request_method".to_owned()), false)
        .expect_one_of_string(
            NoName,
            false,
            vec![
                "GET".to_string(),
                "POST".to_string(),
                "DELETE".to_string(),
                "PUT".to_string(),
                "OPTIONS".to_string(),
            ],
        )
        .delimited_by_space()

        .inline_value_follows(Name("request_uri".to_owned()), false)
        .expect_string(
            NoName,
            false,
            
        )
        .delimited_by_space()

        .inline_value_follows(Name("protocol_version".to_owned()), false)
        .expect_string(NoName,false)
        .delimited_by_newline()
        .build();

    let mut header_placeholder_builder = new_mandatory_spec_builder(Transient("header".to_string()));
    //let mut header_placeholder_builder = header_placeholder_builder.delimited_by_newline();

    let header_place_holder = header_placeholder_builder
        .key_follows(Name("header_name".to_string()), true)
        .expect_string( NoName, false)
        .delimited_by(": ".to_string())
        
        .value_follows(Name("header_value".to_owned()), false)
        .expect_string(NoName, false)
        .delimited_by_newline()
        .build();

    let spec_builder = spec_builder.expect_composite(request_line_placeholder)
    .repeat_many(Name("headers".to_owned()), true, Separator::Delimiter("\r\n".to_owned()),header_place_holder)
    
    .use_spec(Box::new(BodySpec::new(Name("request_body".to_owned()), true)));

    spec_builder.build()
}

pub struct BodySpec{
    spec_meta_data: SpecMetaData,
}

impl BodySpec{
    pub fn new(name: SpecName, optional: bool) -> Self{        
        let spec_meta_data = SpecMetaData::new(name, ValueType::None, optional);
        BodySpec { spec_meta_data: spec_meta_data }
    }
}

impl SimpleValueSpec for BodySpec{}


//impl ToSpecType for BodySpec{}

/* impl SpecTraverse for BodySpec{
    fn traverse(&self, mapper: &mut Box<dyn Mapper>) {
        traverse_spec(self, mapper);
    }
} */

#[async_trait]
impl SpecDeserialize for BodySpec{
    async fn deserialize (
        &self,
        info_provider: &mut ( dyn InfoProvider + Send + Sync ),
        reader: &mut (dyn SpecRead), update_info: bool,
    ) -> Result<Value, ParserError>{
        let content_length_option = info_provider.get_info(&"Content-Length".to_owned());
        return match content_length_option{
            Some(value) => {
                let content_length_str = value.get_string_value();
                let content_length: u32 = content_length_str.unwrap().parse().expect("Failed to parse string to u16 for content-length header");
                NBytesSpec::new(self.spec_meta_data.get_name().clone(), content_length, self.spec_meta_data.is_optional()).deserialize(info_provider, reader, update_info).await
            },
            None => {
                Ok(Value::None)
            },
        }
    }
}

impl Spec for BodySpec{
    fn get_meta_data(&self) -> &SpecMetaData {
        &self.spec_meta_data
    }
}

use crate::core::Anyway;
#[async_trait]
impl SpecSerialize for BodySpec{

    async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ), mapper_context: &mut MapperContext,
            writer: &mut (dyn SpecWrite)
        ) -> Result<(), ParserError>{
            //mapper_context.start_spec(self);
            let spec_name = mapper_context.get_last_available_spec_name();
            if let Some(spec_name) = spec_name{
                let bytes = info_provider.get_info(&spec_name);
                if let Some(value) = bytes{
                    return writer.write_data_bytes(value.get_u8_vec().unwrap()).await;
                }
            }
            Ok(())
        }
    
}

//use crate::core::SpecName::*;
#[allow(unused)]
pub fn build_http_response_protocol() -> ListSpec {
    let root_builder = new_mandatory_spec_builder(
        SpecName::NoName,
    );

    let response_line_placeholder = new_mandatory_spec_builder(SpecName::Transient("response_line".to_string()))
        .inline_value_follows(Name("protocol_version".to_string()), false)
        .expect_string(NoName, false)
        .delimited_by_space()
        .inline_value_follows(Name("status_code".to_string()), false)
        .expect_string(NoName, false)
        .delimited_by_space()
        .inline_value_follows(Name("status_text".to_string()), false)
        .expect_string(NoName, false)
        .delimited_by_newline()
        .build();

    let mut header_placeholder_builder = new_mandatory_spec_builder(Name("header".to_string()));
    let header_place_holder = header_placeholder_builder
        .key_follows(Name("header_name".to_string()), false)
        .expect_string(NoName, false)
        .delimited_by(": ".to_string())
        .value_follows(Name("header_value".to_string()), false)
        .expect_string(NoName, false)
        .delimited_by_newline()
        .build();

    let root_builder = root_builder.expect_composite(response_line_placeholder)
    .repeat_many( Name("headers".to_owned()), true,Separator::Delimiter("\r\n".to_owned()), header_place_holder, )    
    .use_spec(Box::new(BodySpec::new(Name("response_body".to_owned()), true)));
    root_builder.build()
}

#[allow(unused)]
fn build_http_request_info() -> HttpRequestInfo {
    let mut request_info = HttpRequestInfo::default();

    request_info.add_info(
        "RequestMethod".to_string(),
        Value::String("GET".to_string()),
    );
    request_info.add_info("RequestUri".to_string(), Value::String("/".to_string()));
    request_info.add_info(
        "ProtocolVersion".to_string(),
        Value::String("HTTP/1.1".to_string()),
    );
    request_info
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_parsing_get_request() {
        let (mut client, mut server) = tokio::io::duplex(128);
        unsafe {
            let request_str =
                "GET / http/1.1\nContent-Length: 0\nContent-Type: application/json\n\ntest";
            let mut request_str = request_str.to_string();
            let bytes = request_str.as_bytes_mut();
            //let spec = build_http_request_protocol();
            //let mut request_parser = RequestParse::new(spec);
        }
    }
}
