use async_trait::async_trait;
use pin_project::pin_project;
use std::{
     io::{self}, pin::Pin, task::{Context, Poll}
};
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::core::SpecWrite;

use super::ParserError;

#[pin_project]
pub(super) struct ProtocolBuffWriter<R>
where
    R: AsyncWrite + Unpin,
{
    #[pin]
    inner: R,
}

impl <R> ProtocolBuffWriter<R>
where
    R: AsyncWrite + Unpin,
{
    #[allow(unused)]
    pub(super) fn new(inner: R) -> Self {
        ProtocolBuffWriter { inner }
    }
}

impl<R> AsyncWrite for ProtocolBuffWriter<R>
where
    R: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut pinned_self = self.project();
        let pinned_writer = Pin::new(&mut pinned_self.inner);
        pinned_writer.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let mut pinned_self = self.project();
        let pinned_writer = Pin::new(&mut pinned_self.inner);
        pinned_writer.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let mut pinned_self = self.project();
        let pinned_writer = Pin::new(&mut pinned_self.inner);
        pinned_writer.poll_shutdown(cx)
    }
}

#[async_trait]
pub trait PlaceHolderWrite  where Self: AsyncWrite  + Unpin{
     async fn write_string(&mut self, data: String) -> Result<(), ParserError>{
        self.write(data.as_bytes()).await?;
        Ok(())
    }

    async fn write_data_u8(&mut self, data: u8) -> Result<(), ParserError>{
        self.write_u8(data).await?;
        Ok(())
    }

    async fn write_data_u16(&mut self, data: u16) -> Result<(), ParserError>{
        self.write_u16(data).await?;
        Ok(())
    }

    async fn write_data_u32(&mut self, data: u32) -> Result<(), ParserError>{
        self.write_u32(data).await?;
        Ok(())
    }

    async fn write_data_u64(&mut self, data: u64) -> Result<(), ParserError>{
        self.write_u64(data).await?;
        Ok(())
    }

    async fn write_data_i8(&mut self, data: i8) -> Result<(), ParserError>{
        self.write_i8(data).await?;
        Ok(())
    }

    async fn write_data_i16(&mut self, data: i16) -> Result<(), ParserError>{
        self.write_i16(data).await?;
        Ok(())
    }

    async fn write_data_i32(&mut self, data: i32) -> Result<(), ParserError>{
        self.write_i32(data).await?;
        Ok(())
    }

    async fn write_data_i64(&mut self, data: i64) -> Result<(), ParserError>{
        self.write_i64(data).await?;
        Ok(())
    }

    async fn write_data_bytes(&mut self, data: &[u8]) -> Result<(), ParserError>{
        self.write(data).await?;
        Ok(())
    }

}

#[async_trait]
impl<R> PlaceHolderWrite for ProtocolBuffWriter<R>
where
    R: AsyncWrite + Unpin{}

impl<R> SpecWrite for ProtocolBuffWriter<R>
where
    R: AsyncWrite + Unpin + Send + Sync
{}


impl<R> ProtocolBuffWriter<R>
where
    R: AsyncWrite + Unpin,
{   


    /* #[allow(unused)]
    pub(super) async fn write_composite<RI>(
        &mut self,
        spec: &ListSpec,
        request_info: &RI,
        key:  Option<&String>,
        
    ) -> Result<(), ParserError>
    where
        RI: InfoProvider,
    {
        match spec {
            
            Some(constituents) => {
                //for i in 0..constituents.len() {
                let  mut i = 0;
                let mut key_option: Option<&String>;
                loop {
                    //let mut value: Option<String> = None;
                    if i >= constituents.len() {
                        return Ok(());
                    }
                    let constituent = &constituents[i];
                    match &constituent.place_holder_type {
                        PlaceHolderType::Composite => {
                            Box::pin(self.write_composite(constituent, request_info, key,))
                                .await?;
                        }

                        #[allow(unused)]
                        PlaceHolderType::RepeatN(n) => {
                            Box::pin(self.write_composite(constituent, request_info, None,))
                                .await?;
                        }

                        PlaceHolderType::RepeatMany => {                            
                            let header_name = match &constituent.name {
                                crate::core::PlaceHolderIdentifier::Name(name) => {
                                    Some(name.to_owned())
                                }

                                _ => None,
                            };

                            if let Some(header_name) = header_name {
                                if let Some(keys) = request_info.get_keys_by_group_name(header_name)
                                {
                                    for key in &keys {
                                        key_option = Some(key);
                
                                        Box::pin(self.write_composite(
                                            constituent,
                                            request_info,
                                            key_option,                                                
                                        ))
                                        .await?;
                    
                                    }
                                }
                            }
                        }
                        PlaceHolderType::ExactString(input) => {
                            self.prepare_and_write_data(request_info, key, constituent, Some(input)).await?;
                            return Ok(());
                        }
                        PlaceHolderType::AnyString => {
                            self.prepare_and_write_data(request_info, key, constituent, None).await?;
                        }
                        PlaceHolderType::StreamValue(name) => {
                            self.prepare_and_write_data(request_info, key, constituent, Some(name)).await?;
                        }

                        #[allow(unused)]
                        PlaceHolderType::OneOf(items) => {
                            self.prepare_and_write_data(request_info, key, constituent, None).await?;
                        }

                        PlaceHolderType::BytesOfSizeFromHeader(header) => {
                            self.prepare_and_write_data(request_info, key, constituent, None).await?;
                        }

                        PlaceHolderType::BytesOfSizeN(_) => {
                            self.prepare_and_write_data(request_info, key, constituent, None).await?;
                        },

                        PlaceHolderType::Bytes => {
                            self.prepare_and_write_data(request_info, key, constituent, None).await?;
                        },

                        PlaceHolderType::Space => {
                            self.inner.write(b" ").await?;
                        },
                        PlaceHolderType::NewLine => {
                            self.inner.write(b"\n").await?;
                        }
                        PlaceHolderType::Delimiter(delim) => {
                            self.inner.write(delim.as_bytes()).await?;
                        }
                    }       
                    i += 1;
                }
            }
        }
        self.flush().await?;
        Ok(())  
    } */

    /* async fn prepare_and_write_data<RI:InfoProvider>(&mut self, request_info: &RI, mut data:  Option<&String>, constituent: &Placeholder, overriding_data: Option<&String>) -> Result<(), ParserError> {
        if data.is_none() && overriding_data.is_none() {
            match &constituent.name {
               
                crate::core::PlaceHolderIdentifier::InlineKeyWithValue(_key) => {                    
                }

                crate::core::PlaceHolderIdentifier::Name(_key) => {                    
                }
                _ =>{
                    return Err(ParserError::MissingData);
                }
            }
            
        }


        
        match &constituent.name {
            crate::core::PlaceHolderIdentifier::Name(_) => {                                    
                if let Some(data) = data {
                    self.inner.write(data.as_bytes()).await?;
                    return Ok(());
                } else {
                    return Err(ParserError::MissingKey);
                }
            }
            crate::core::PlaceHolderIdentifier::Key => {        
                if !overriding_data.is_none(){
                    data = overriding_data;
                }
                self.inner.write(data.unwrap().as_bytes()).await?;
                return Ok(());
            }
            crate::core::PlaceHolderIdentifier::Value => {
                
                if overriding_data.is_none(){
                    let value = request_info.get_info(data.unwrap());
                    if let Some(value) = value {
                        value.write(&mut self.inner).await?;
                        return Ok(());
                    } else {
                        return Err(ParserError::MissingValue(data.unwrap().to_owned()));
                    }
                }else{
                    self.inner.write(overriding_data.unwrap().as_bytes()).await?;
                    Ok(())
                }
            }
            crate::core::PlaceHolderIdentifier::InlineKeyWithValue(key) => {
                if overriding_data.is_none(){
                    let value = request_info.get_info(key);
                    if let Some(value) = value {
                        value.write(&mut self.inner).await?;
                        return Ok(());
                    } else if !constituent.optional {
                        return Err(ParserError::MissingValue(data.unwrap().to_owned()));
                    }else {
                        return Ok(());
                    }
                }else{
                    self.inner.write(overriding_data.unwrap().as_bytes()).await?;
                    Ok(())
                }
            }
        }
    } */
}


#[cfg(test)]
mod tests {
    

    
    

    //#![debugger_visualizer(natvis_file = "../Foo.natvis")]    
    use std::collections::HashMap;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use crate::core::protocol_writer::ProtocolBuffWriter;
    use crate::core::PlaceHolderIdentifier::{InlineKeyWithValue, Name};
    use crate::core::{
        new_spec_builder, CustomSpecBuilder, DelimitedStringSpecBuilder, DelimiterBuilder, InfoProvider, InlineValueBuilder, KeySpecBuilder, Mapper, ProtoSpecBuilder, RepeatBuilder, StringSpecBuilder, Value, ValueBuilder
    };
    use crate::http::BodySpec;
    use crate::mapping_extractor::DefaultMapper;
    

    pub struct TestRequestInfo(HashMap<String, Value>, Box<dyn Mapper>);

    impl Default for TestRequestInfo{
        fn default() -> Self {
            TestRequestInfo(HashMap::new(), Box::new(DefaultMapper::new()))
        }
    }

    impl TestRequestInfo {
        pub fn new() -> Self {
            let mut r = TestRequestInfo(HashMap::new(), Box::new(DefaultMapper::new()));
            r.0.insert("data".to_owned(), Value::String("test".to_string()));
            r.0.insert("data1".to_owned(), Value::String("test1".to_string()));
            return r;
        }
    }

    impl InfoProvider for TestRequestInfo {
        fn add_info(&mut self, key: String, value: Value) {
            self.0.insert(key, value);
        }

        fn get_info(&self, key: &String) -> Option<&crate::core::Value> {            
            self.0.get(key)
        }
        
        
        
        fn get_mapper_mut(&mut self) ->&mut Box<dyn crate::core::Mapper> {
            &mut self.1
        }
        
        fn get_mapper(&self) ->&Box<dyn Mapper> {
             &self.1
        }
    }

    

    /* #[tokio::test]
    async fn test_write_composite() {        
        let root_placeholder =
            new_spec_builder("root".to_string(), false);

        

        let request_line_placeholder = new_spec_builder("request_line".to_string(), false);
        let request_line_spec = request_line_placeholder
            .inline_value_follows("request_method".to_string(), false)
            .expect_one_of_string(
                None,
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
            .inline_value_follows("request_uri".to_string(),false)
            .expect_string(None,false)
            .delimited_by_space()
            .inline_value_follows("protocol_version".to_string(),false)
            .expect_string(None,false)
            .delimited_by_newline()
            .build();

        let mut header_placeholder_builder = new_spec_builder("header".to_string(), false);
        let header_place_holder = header_placeholder_builder
            .key_follows("header_name".to_string(), false)
            .expect_string(None, false)
            .delimited_by_space()
            .value_follows("header_value".to_string(), false)
            .expect_string(None, false)
            .delimited_by_newline()
            .build();

        let spec = root_placeholder
            .use_spec(Box::new(request_line_spec))
            .repeat_many(Some("headers".to_string()), false, header_place_holder)
            .expect_newline()
            .use_spec(Box::new(BodySpec::new(None, true)))
            .build();
    
        let mut request_info = TestRequestInfo::new();
        request_info.add_info("request_method".to_owned(), Value::String("GET".to_owned()));
        request_info.add_info("request_uri".to_owned(), Value::String("/".to_owned()));
        request_info.add_info("protocol_version".to_owned(), Value::String("HTTP/1.1".to_owned()));

        request_info.add_info("Content-Length".to_owned(), Value::String("100".to_owned()));
        
        request_info.add_info("body".to_owned(), Value::U8Vec("hello".as_bytes().to_vec()));
        let (mut receiver, mut sender) = tokio::io::simplex(64);
                
        ProtocolBuffWriter::new(&mut sender)
            .write_composite(&placehoder, &mut request_info, None)
            .await
            .unwrap();
        sender.flush().await.unwrap();
        sender.shutdown().await.unwrap();
        let mut result = String::new();
        let _r = receiver.read_to_string(&mut result).await;
        assert!(result == "GET / HTTP/1.1\ndata1: test1\ndata: test\n\nhello" || result == "GET / HTTP/1.1\ndata: test\ndata1: test1\n\nhello");
        println!("Result: {}", result);

        
    } */
}
