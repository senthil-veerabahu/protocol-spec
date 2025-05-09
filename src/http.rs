#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::core::PlaceHolderIdentifier::{InlineKeyWithValue, Name};
use crate::core::*;
use std::collections::HashMap;
use std::io::Error;


struct HttpRequestInfo {
    request_type: ValueType,
    headers: HashMap<String, ValueType>,
    protocol_version: Option<ValueType>,
    request_uri: Option<ValueType>,
    request_body: Option<ValueType>,
}

impl Default for HttpRequestInfo {
    fn default() -> Self {
        HttpRequestInfo {
            request_type: ValueType::String("GET".to_string()),
            headers: Default::default(),
            protocol_version: None,
            request_uri: None,
            request_body: Default::default(),
        }
    }
}

/* struct HttpFactory<REQI:RequestInfo, RESI:ResponseInfo>((REQI, RESI));

impl <REQI:RequestInfo, RESI:ResponseInfo> RequestFactory for HttpFactory<RESI, REQI> {
    

    fn create_request(&self) -> Self::REQ {
        HttpRequestInfo::default()
    }

    fn create_response(&self) -> Self::RES {
        HttpResponseInfo::default()
    }
    
    fn get_request_spec(&self) -> &Placeholder {
        todo!()
    }
    
    fn create_request_info(&self) -> REQI {
        todo!()
    }
    
    fn create_request_serializer(&self) -> REQSER {
        todo!()
    }
    
    fn create_request_handler(&self) -> &REQH {
        todo!()
    }
    
    fn create_error_request_handler(&self) -> REQERRH {
        todo!()
    }
} */

//struct HttpConfig;
/* impl <R, W> ProtocolConfig<R, W> for HttpConfig {
    type REQF = ;

    type RESF;

    type REQI = HttpRequestInfo;

    type RESI = HttpResponseInfo;

    type REQSER;

    type RESSER;

    type REQH;

    type RESH;

    type REQERRH;

    type RESERRH;
} */

impl InfoProvider for HttpRequestInfo {
    fn get_info(&self, key: &String) -> Option<&ValueType> {
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

    fn add_info(&mut self, key: String, value: ValueType) {
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

    fn get_info_mut(&mut self, key: &String) -> Option<&mut ValueType> {
        todo!()
    }
}

impl HttpRequestInfo {
    #![allow(unused)]
    fn set_request_type(&mut self, request_type: String) {
        self.request_type = ValueType::String(request_type);
    }
}

impl From<std::io::Error> for ParserError {
    fn from(error: Error) -> Self {
        ParserError::IOError { error }
    }
}





#[allow(unused)]
fn build_http_request_protocol() -> Placeholder {
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
    spec_builder.expect_bytes(InlineKeyWithValue("requeset_body".to_string()));

    spec_builder.build()
}



#[allow(unused)]
fn build_http_response_protocol() -> Placeholder {
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
    spec_builder.expect_bytes(InlineKeyWithValue("body".to_string()));

    spec_builder.build()
}

#[allow(unused)]
fn build_http_request_info(root_place_holder: Placeholder) -> HttpRequestInfo {
    let mut request_info = HttpRequestInfo::default();

    request_info.add_info(
        "RequestMethod".to_string(),
        ValueType::String("GET".to_string()),
    );
    request_info.add_info("RequestUri".to_string(), ValueType::String("/".to_string()));
    request_info.add_info(
        "ProtocolVersion".to_string(),
        ValueType::String("http/1.1".to_string()),
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
