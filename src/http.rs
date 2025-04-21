#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::core::*;
use crate::core::PlaceHolderIdentifier::{InlineKeyWithValue, Key, Name};
use crate::utils::pop_newline;
use std::collections::HashMap;
use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

struct HttpParser;

struct HttpRequestInfo {
    request_type: ValueType,
    headers: HashMap<String, ValueType>,
    protocol_version: Option<ValueType>,
    request_uri: Option<ValueType>,
    protocol_name: Option<ValueType>,
    request_body: Option<ValueType>,
}

impl Default for HttpRequestInfo {
    fn default() -> Self {
        HttpRequestInfo {
            request_type: ValueType::String("GET".to_string()),
            headers: Default::default(),
            protocol_version: None,
            request_uri: None,
            protocol_name: None,
            request_body: Default::default(),
        }
    }
}

impl RequestInfo for HttpRequestInfo {
    fn get_info(&self, key: String) -> Option<&ValueType> {
        let key_ref = key.as_str();
        match key_ref {
            "RequestMethod" => {
                return Some(&self.request_type);
            }
            "ProtocolVersion" => {
                return self.protocol_version.as_ref();
            }
            "RequestUri" => {
                return self.request_uri.as_ref();
            }
            "ProtocolName" => {
                return self.protocol_name.as_ref();
            }
            "RequestBody" => {
                return self.request_body.as_ref();
            }
            _ => {
                return self.headers.get(key_ref).clone();
            }
        }
    }

    fn add_info(&mut self, key: String, value: ValueType) {
        match key.as_str() {
            "RequestMethod" => {
                self.request_type = value;
            }
            "ProtocolVersion" => {
                self.protocol_version = Some(value);
            }
            "ProtocolName" => {
                self.protocol_name = Some(value);
            }
            "RequestUri" => {
                self.request_uri = Some(value);
            }
            "RequestBody" => {
                self.request_body = Some(value);
            }

            _ => {
                self.headers.insert(key.to_string(), value);
            }
        }
    }
}

impl HttpRequestInfo {
    #![allow(unused)]
    fn set_request_type(&mut self, request_type: String)
    
    {
        self.request_type = ValueType::String(request_type);
    }
}

impl From<std::io::Error> for ParserError {
    fn from(error: Error) -> Self {
        ParserError::IOError { error }
    }
}



impl HttpParser {}




impl HttpParser {
    #![allow(unused_mut)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    async fn parse_body<'a, Reader: AsyncRead + Unpin>(
        &self, _reader: &mut BufReader<Reader>

    ) -> Result<Vec<u8>, ParserError> {
        let buf = vec![];
        Ok(buf)
    }
}


#[allow(unused)]
fn build_http_protocol() -> Placeholder {
    let root_placeholder = Placeholder::new(Name("root".to_string()), None, PlaceHolderType::Composite, false);

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

    spec_builder.expect_newline();

    spec_builder.expect_composite(request_line_placeholder, "first_line".to_string());

    let header_placeholder = SpecBuilder::new_composite("headers".to_string(), false)
        .expect_string(Key, false)
        .expect_delimiter(":".to_string())
        .expect_space()
        .expect_value_string(false)
        .expect_newline()
        .build();

    let headers_placeholder = Placeholder::new(
        Name("Headers".to_string()),
        Some(vec![header_placeholder]),
        PlaceHolderType::RepeatMany,
        false,
    );

    spec_builder.expect_repeat_many(headers_placeholder, "headers".to_string());
    spec_builder.expect_stream(InlineKeyWithValue("body".to_string()), false);
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

impl RequestParser for HttpParser {
    async fn parse_request<RI: RequestInfo, Reader: AsyncRead + Unpin>(
        &self,
        reader: Reader,
    ) -> Result<RI, ParserError> {
        let mut async_buf_reader = BufReader::new(reader);
        let mut http_request_info = RI::default();
        let mut first_line = String::new();

        let _len = async_buf_reader.read_line(&mut first_line).await?; //{
        pop_newline(&mut first_line);
        let mut first_line_parts = first_line.split::<&str>(" ");
        //let request_method = first_line_parts.next();
        match (
            first_line_parts.next(),
            first_line_parts.next(),
            first_line_parts.next(),
        ) {
            (Some(request_method), Some(request_uri), Some(protocol_version)) => {
                http_request_info.add_info(
                    "RequestMethod".to_string(),
                    ValueType::String(request_method.to_string()),
                );
                http_request_info.add_info(
                    "RequestUri".to_string(),
                    ValueType::String(request_uri.to_string()),
                );
                http_request_info.add_info(
                    "ProtocolVersion".to_string(),
                    ValueType::String(protocol_version.to_string()),
                );
            }
            (_, _, _) => {
                return Err(ParserError::TokenExpected { line_index:0, char_index: 0, message: "Expected Http Request first line  of the form <requestmethod> <httpversion> <requesturi>".to_string() });
            }
        }
        //}

        //Expect empty line
        let len = async_buf_reader.read_line(&mut first_line).await?;
        assert_eq!(len, 1);

        loop {
            let mut line = String::new();
            let len = async_buf_reader.read_line(&mut line).await?;
                if len == 0 {
                    return Ok(http_request_info);
                } else {
                    pop_newline(&mut line);
                    println!("read line is {}", line);
                    if line.len() == 0 {
                        let body: Vec<u8> = self.parse_body(&mut async_buf_reader).await?;
                        http_request_info
                            .add_info("RequestBody".to_string(), ValueType::U8Vec(body));
                        continue;
                    }
                    let mut parts = line.split(":");
                    let key_option = parts.next();
                    let value_option = parts.next();
                    match (key_option, value_option) {
                        (None, None) => {}
                        (Some(key), None) => {
                            return Err(ParserError::TokenExpected {line_index:0,  char_index: 0, message: format!("Expected http header {} to be of the form <requestmethod> <httpversion> <requesturi>", key) });
                        }
                        (Some(key), Some(value)) => {
                            http_request_info.add_info(
                                key.to_string(),
                                ValueType::String(value.trim().to_string()),
                            );
                        }
                        _ => {}
                    }
                }
            /* } else {
                  return Err(ParserError::InvalidToken {
                      position: 0,
                      message: "Unexpected token or e".to_string(),
                  });
              }*/
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{ParserError, RequestInfo, RequestParser, ValueExtractor};
    use crate::http::{HttpParser, HttpRequestInfo};
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_parsing_get_request() {
        let (mut client, mut server) = tokio::io::duplex(128);
        unsafe {
            let request_str =
                "GET / http/1.1\n\nContent-Length: 0\nContent-Type: application/json\n\ntest";
            let mut request_str = request_str.to_string();
            let bytes = request_str.as_bytes_mut();
            let _ = client.write_all(bytes).await;
            let _ = client.flush().await;
            //close stream so that the reader completes reading line
            drop(client);

            let parser = HttpParser {};
            let h: Result<HttpRequestInfo, ParserError> = parser.parse_request(server).await;
            match h {
                Ok(info) => {
                    assert_eq!(
                        info.get_info("RequestUri".to_string())
                            .unwrap()
                            .get_string_value()
                            .unwrap(),
                        "/"
                    );
                    assert_eq!(
                        info.get_info("ProtocolVersion".to_string())
                            .unwrap()
                            .get_string_value()
                            .unwrap(),
                        "http/1.1"
                    );
                    assert_eq!(
                        info.get_info("RequestMethod".to_string())
                            .unwrap()
                            .get_string_value()
                            .unwrap(),
                        "GET"
                    );

                    assert_eq!(
                        info.get_info("Content-Length".to_string())
                            .unwrap()
                            .get_string_value()
                            .unwrap(),
                        "0"
                    );
                    assert_eq!(
                        info.get_info("Content-Type".to_string())
                            .unwrap()
                            .get_string_value()
                            .unwrap(),
                        "application/json"
                    );
                    assert_eq!(
                        String::from_utf8(
                            (info
                                .get_info("RequestBody".to_string())
                                .unwrap()
                                .get_u8_vec()
                                .unwrap())
                            .clone()
                        )
                        .unwrap(),
                        "test"
                    );
                }
                Err(error) => {
                    println!("error {}", error);
                }
            }
        }
    }
}
