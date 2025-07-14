#![allow(unused_variables)]
#![allow(unused_mut)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::to_string;


use crate::core::PlaceHolderIdentifier::{InlineKeyWithValue, Name};
use crate::core::*;
use paste::paste;
use std::collections::HashMap;
use std::io::Error;
use std::str::{self, from_utf8, Utf8Error};

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
        ValueType::String($value)
    };

    (i64, $value:expr) => {
        ValueType::SignedNumber64($value)
    };

    (u64, $value:expr) => {
        ValueType::UnSignedNumber64($value)
    };

    (u32, $value:expr) => {
        ValueType::UnSignedNumber32($value)
    };

    (i16, $value:expr) => {
        ValueType::SignedNumber16($value)
    };

    (u16, $value:expr) => {
        ValueType::UnSignedNumber16($value)
    };

    

   (Vecu8,  $value:expr) =>{
        ValueType::U8Vec($value)
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
}

impl Default for HttpRequestInfo {
    fn default() -> Self {
        HttpRequestInfo {
            request_type: Value::String("GET".to_string()),
            headers: Default::default(),
            protocol_version: None,
            request_uri: None,
            request_body: Default::default(),
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
}

impl ResponseInfo for HttpResponseInfo {}

impl Default for HttpResponseInfo {
    fn default() -> Self {
        HttpResponseInfo {
            status_code: Value::String("200".to_string()),
            headers: Default::default(),
            protocol_version: Some(Value::String("HTTP/1.1".to_string())),
            status_text: Value::String("Ok".to_string()),
            response_body: Default::default(),
        }
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

pub struct HttpRequestFactory(pub Placeholder);

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
                    .with_status_text("Ok".to_owned())
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
                    .with_status_text("Ok".to_owned())                    
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
    fn get_request_spec(&self) -> &Placeholder {
        &self.0
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

pub struct HttpResponseFactory(pub Placeholder);

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
    fn get_response_spec(&self) -> &Placeholder {
        &self.0
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

    fn create_error_responset_handler(&self) -> HttpResponseHandler {
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
            "status_code" => {
                return Some(&self.status_code);
            }
            "protocol_version" => {
                return self.protocol_version.as_ref();
            }
            "status_text" => {
                return Some(&self.status_text);
            }

            "response_body" => {
                return self.response_body.as_ref();
            }
            _ => {
                return self.headers.get(key_ref).clone();
            }
        }
    }

    fn add_info(&mut self, key: String, value: Value) {
        match key.as_str() {
            "status_text" => {
                self.status_text = value;
            }
            "protocol_version" => {
                self.protocol_version = Some(value);
            }

            "status_code" => {
                self.status_code = value;
            }
            "response_body" => {
                self.response_body = Some(value);
            }

            _ => {
                self.headers.insert(key.to_string(), value);
            }
        }
    }

    fn get_keys_by_group_name(&self, name: String) -> Option<Vec<&String>> {
        Some(self.headers.keys().collect())
    }

    fn get_info_mut(&mut self, key: &String) -> Option<&mut Value> {
        todo!()
    }
    
    fn has_all_data(&self) -> bool {
        if let Some(content_length) =  self.get_content_length() {
            if let Some(body_value_type) = self.get_info(&"response_body".to_owned()) {
                if let Some(body) = body_value_type.get_u8_vec() {
                    if content_length == body.len().to_string() {
                        return true;
                    }
                }
            }
            return false
        }
        return self.headers.capacity() > 0;
    }
}

impl InfoProvider for HttpRequestInfo {
    fn get_info(&self, key: &String) -> Option<&Value> {
        let key_ref = key.as_str();
        match key_ref {
            "request_method" => {
                return Some(&self.request_type);
            }
            "protocol_version" => {
                return self.protocol_version.as_ref();
            }
            "request_uri" => {
                return self.request_uri.as_ref();
            }

            "request_body" => {
                return self.request_body.as_ref();
            }
            _ => {
                return self.headers.get(key_ref).clone();
            }
        }
    }

    fn add_info(&mut self, key: String, value: Value) {
        match key.as_str() {
            "request_method" => {
                self.request_type = value;
            }
            "protocol_version" => {
                self.protocol_version = Some(value);
            }

            "request_uri" => {
                self.request_uri = Some(value);
            }
            "request_body" => {
                self.request_body = Some(value);
            }

            _ => {
                self.headers.insert(key.to_string(), value);
            }
        }
    }

    fn get_keys_by_group_name(&self, name: String) -> Option<Vec<&String>> {
        Some(self.headers.keys().collect())
    }

    fn get_info_mut(&mut self, key: &String) -> Option<&mut Value> {
        todo!()
    }

    fn has_all_data(&self) -> bool {
        if let Some(content_length) =  self.get_content_length() {
            if let Some(body_value_type) = self.get_info(&"request_body".to_owned()) {
                if let Some(body) = body_value_type.get_u8_vec() {
                    if content_length == body.len().to_string() {
                        return true;
                    }
                }
            }
            return false
        }
        return self.headers.capacity() > 0;
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
pub fn build_http_request_protocol() -> Placeholder {
    let root_placeholder = Placeholder::new(
        Name("root".to_string()),
        None,
        PlaceHolderType::Composite,
        false,
    );

    let mut spec_builder = SpecBuilder(root_placeholder);

    let request_line_placeholder = SpecBuilder::new_composite("request_line".to_string(), false)
        .expect_one_of_string(
            vec![
                "GET".to_string(),
                "POST".to_string(),
                "DELETE".to_string(),
                "PUT".to_string(),
                "OPTIONS".to_string(),
            ],
            InlineKeyWithValue("request_method".to_string()),
            false,
        )
        .expect_space()
        .expect_string(InlineKeyWithValue("request_uri".to_string()), false)
        .expect_space()
        .expect_string(InlineKeyWithValue("protocol_version".to_string()), false)
        .expect_newline()
        .build();

    let mut header_placeholder_builder = SpecBuilder::new_composite("header".to_string(), false);
    let header_place_holder = header_placeholder_builder
        .expect_string(crate::core::PlaceHolderIdentifier::Key, false)
        .expect_delimiter(": ".to_string())
        .expect_string(crate::core::PlaceHolderIdentifier::Value, false)
        .expect_newline()
        .build();

    spec_builder.expect_composite(request_line_placeholder, "first_line".to_owned());
    spec_builder.expect_repeat_many(header_place_holder, "headers".to_owned());
    spec_builder.expect_newline();
    spec_builder.expect_bytes_of_size_from_header(InlineKeyWithValue("request_body".to_string()), "Content-Length".to_owned(),true);

    spec_builder.build()
}

#[allow(unused)]
pub fn build_http_response_protocol() -> Placeholder {
    let root_placeholder = Placeholder::new(
        Name("root".to_string()),
        None,
        PlaceHolderType::Composite,
        false,
    );

    let mut spec_builder = SpecBuilder(root_placeholder);

    let response_line_placeholder = SpecBuilder::new_composite("response_line".to_string(), false)
        .expect_string(InlineKeyWithValue("protocol_version".to_string()), false)
        .expect_space()
        .expect_string(InlineKeyWithValue("status_code".to_string()), false)
        .expect_space()
        .expect_string(InlineKeyWithValue("status_text".to_string()), false)
        .expect_newline()
        .build();

    let mut header_placeholder_builder = SpecBuilder::new_composite("header".to_string(), false);
    let header_place_holder = header_placeholder_builder
        .expect_string(crate::core::PlaceHolderIdentifier::Key, false)
        .expect_delimiter(": ".to_string())
        .expect_string(crate::core::PlaceHolderIdentifier::Value, false)
        .expect_newline()
        .build();

    spec_builder.expect_composite(response_line_placeholder, "first_line".to_owned());
    spec_builder.expect_repeat_many(header_place_holder, "headers".to_owned());
    spec_builder.expect_newline();
    spec_builder.expect_bytes_of_size(InlineKeyWithValue("response_body".to_string()),10,true);

    spec_builder.build()
}

#[allow(unused)]
fn build_http_request_info(root_place_holder: Placeholder) -> HttpRequestInfo {
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
