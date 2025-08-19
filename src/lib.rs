//#![debugger_visualizer(natvis_file = "./Foo.natvis")]


pub mod core {
    use crate::core::protocol_reader::{Marker, ReadBytesSize};
    use crate::core::protocol_writer::PlaceHolderWrite;
    use crate::core::protocol_reader::PlaceHolderRead;    
    use async_trait::async_trait;
    use derive_builder::Builder;
    use protocol_reader::ProtocolBuffReader;
    use protocol_reader::{ MarkAndRead};

    use protocol_writer::ProtocolBuffWriter;
    
    
    
    
    use std::marker::PhantomData;
    use std::{
        fmt::{Debug, Display, Formatter}, mem::{self}, str::Utf8Error
    };
    use tokio::{
        io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
    };


    #[allow(dead_code)]
    pub trait ProtocolInfo {
        fn get_name() -> String;
        fn get_version() -> String;
        fn get_transport_type() -> Transport;
        fn get_format() -> ProtocolFormat;
    }

    #[allow(unused)]
    #[derive(Debug)]
    pub enum ParserError {
        InvalidPlaceHolderTypeFound {
            line_index: usize,
            char_index: usize,
            message: String,
        },
        TokenExpected {
            line_index: usize,
            char_index: usize,
            message: String,
        },
        InvalidToken {
            line_index: usize,
            char_index: usize,
            message: String,
        },
        UnexpectedKeyOrValue {          
            message: String,
        },
        MissingKey,
        MissingData(String),
        MissingValue(String),
        SerdeError(String),
        Utf8Error(Utf8Error),
        EndOfStream,
        InvalidMarker {
            line_index: usize,
            char_index: usize,
            message: String,
        },

        IOError {
            error: std::io::Error,
        },
    }

    impl<'l> Display for ParserError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ParserError::TokenExpected {
                                            line_index: line_pos,
                                            char_index: position,
                                            message,
                                        } => {
                                            write!(
                                                f,
                                                "Token expected at line {} char_pos {} {}",
                                                line_pos, position, message
                                            )
                                        }
                ParserError::InvalidToken {
                                            line_index: line_pos,
                                            char_index: char_pos,
                                            message,
                                        } => {
                                            write!(
                                                f,
                                                "Invalid token at line {}  position {} {}",
                                                line_pos, char_pos, message
                                            )
                                        }
                ParserError::IOError { error } => {
                                            write!(f, "IO Error {}", error)
                                        }
                ParserError::MissingKey => write!(
                                            f,
                                            "Key value pair is expected. But key is missing, only value is present "
                                        ),
                ParserError::InvalidPlaceHolderTypeFound {
                                            line_index: line_pos,
                                            char_index: char_pos,
                                            message,
                                        } => {
                                            write!(
                                                f,
                                                "Invalid placeholder type found at line {}  position {} {}",
                                                line_pos, char_pos, message
                                            )
                                        }
                ParserError::MissingData(key) => {
                                            write!(f, "Expected data for key {} but found none whle writing to writer", key)
                                        }
                ParserError::MissingValue(key) => write!(
                                            f,
                                            "Expected value for key {} but found none whle writing to writer",
                                            key
                                        ),
                ParserError::Utf8Error(_key) => write!(
                                            f,
                                            "Expected value is not a valid  utf-8 data",                    
                                        ),
                ParserError::SerdeError(_key) => write!(
                                            f,
                                            "Expected value is not a valid  utf-8 data",                    
                                        ),
                ParserError::EndOfStream => write!(
                                f,
                                "End of stream reached while parsing data. Expected more data to be present.",                    
                            ),
                ParserError::InvalidMarker { line_index, char_index, message } => write!(
                    f,
                    "Invalid Marker provided during mark/reset operation at line {} char_pos {}: {}", line_index, char_index, message                       
                ),
                Self::UnexpectedKeyOrValue { message } => {
                    write!(f, "Unexpected key or value found: {}", message)
                }
            }
        }
    }

    /*fn get_value_from_enum<T>(val_type: ValueType) -> Box<dyn ValueExtractor<T>> {
        match val_type {
            ValueType::String(s) => { return Box::new(Value(s)); }
            ValueType::SignedNumber64(data) => { return Box::new(Value(data)); }
            ValueType::UnSignedNumber64(data) => { return Box::new(Value(data)); }
            ValueType::UnSignedByteSlice(data) => { return Box::new(Value(data)); }
            ValueType::None => { Box::new(Value(())) }
        }
    }*/

    #[allow(unused)]
    pub trait ValueExtractor<'a> {
        fn get_string_value_unchecked(&self) -> Result<String, ParserError>;
        fn get_signed_num_64_value_unchecked(&self) -> Result<i64, ParserError>;
        fn get_unsigned_num_64_value_unchecked(&self) -> Result<u64, ParserError>;
        fn get_unsigned_num_32_value_unchecked(&self) -> Result<u32, ParserError>;
        fn get_signed_num_16_value_unchecked(&self) -> Result<i16, ParserError>;
        fn get_unsigned_num_16_value_unchecked(&self) -> Result<u16, ParserError>;
        fn get_u8_vec_unchecked(&self) -> Result<&Vec<u8>, ParserError>;

        


        fn get_string_value(&self) -> Option<String>;
        fn get_signed_num_64_value(&self) -> Option<i64>;
        fn get_unsigned_num_64_value(&self) -> Option<u64>;
        fn get_unsigned_num_32_value(&self) -> Option<u32>;

        fn get_signed_num_16_value(&self) -> Option<i16>;
        fn get_unsigned_num_16_value(&self) -> Option<u16>;
        //fn get_unsigned_byte_slice(&self) -> Option<&'a [u8]>;

        fn get_u8_vec(&self) -> Option<&Vec<u8>>;
    }

    impl<'a> ValueExtractor<'a> for Value {
        fn get_string_value(&self) -> Option<String> {
            return match &self {
                Value::String(ref data) => Some(data.clone()),
                Value::UnSignedNumber16(ref data) => Some(data.to_string()),
                Value::UnSignedNumber32(ref data) => Some(data.to_string()),
                Value::UnSignedNumber64(ref data) => Some(data.to_string()),
                Value::SignedNumber16(ref data) => Some(data.to_string()),
                Value::SignedNumber64(ref data) => Some(data.to_string()),

                _ => {
                    return None;
                }
            };
        }

        fn get_signed_num_64_value(&self) -> Option<i64> {
            return match self {
                Value::SignedNumber64(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_64_value(&self) -> Option<u64> {
            return match self {
                Value::UnSignedNumber64(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_32_value(&self) -> Option<u32> {
            return match self {
                Value::UnSignedNumber32(data) => Some(*data),
                Value::String(data) => Some(data.parse::<u32>().unwrap()),

                _ => {
                    return None;
                }
            };
        }

        fn get_signed_num_16_value(&self) -> Option<i16> {
            return match self {
                Value::SignedNumber16(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        fn get_unsigned_num_16_value(&self) -> Option<u16> {
            return match self {
                Value::UnSignedNumber16(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        }

        /* fn get_unsigned_byte_slice(&self) -> Option<&'a [u8]> {
            return match self {
                ValueType::UnSignedByteSlice(data) => Some(*data),

                _ => {
                    return None;
                }
            };
        } */

        fn get_u8_vec(&self) -> Option<&Vec<u8>> {
            return match self {
                Value::U8Vec(data) => Some(data),

                _ => {
                    return None;
                }
            };
        }
        
        fn get_string_value_unchecked(&self) -> Result<String, ParserError> {
            match self.get_string_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get String value from {:?}", self)))
            }
        }
        
        fn get_signed_num_64_value_unchecked(&self) -> Result<i64, ParserError> {
            match self.get_signed_num_64_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get signed 64 value from {:?}", self)))
            }
        }
        
        fn get_unsigned_num_64_value_unchecked(&self) -> Result<u64, ParserError> {
            match self.get_unsigned_num_64_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get unsigned 64 value from {:?}", self)))
            }
        }

        fn get_unsigned_num_32_value_unchecked(&self) -> Result<u32, ParserError> {
            match self.get_unsigned_num_32_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get unsigned 32 value from {:?}", self)))
            }
        }

        fn get_signed_num_16_value_unchecked(&self) -> Result<i16, ParserError> {
            match self.get_signed_num_16_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get signed 64 value from {:?}", self)))
            }
        }
        
        fn get_unsigned_num_16_value_unchecked(&self) -> Result<u16, ParserError> {
            match self.get_unsigned_num_16_value(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get unsigned 64 value from {:?}", self)))
            }
        }
        
        fn get_u8_vec_unchecked(&self) -> Result<&Vec<u8>, ParserError> {
            match self.get_u8_vec(){
                Some(data) => Ok(data),
                None => Err(ParserError::MissingValue(format!("unable to get vec of bytes value from {:?}", self)))
            }
        }
    }

    //#[derive(Clone)]
    #[allow(unused)]
    #[derive(Debug)]
    pub enum Value {
        String(String),
        SignedNumber64(i64),
        UnSignedNumber64(u64),
        UnSignedNumber32(u32),
        SignedNumber16(i16),
        UnSignedNumber16(u16),
        //UnSignedByteSlice([u8]),
        U8Vec(Vec<u8>),
        //StreamData(&'static mut protocol_reader::ProtoStream<BufReader<tokio::net::TcpStream>>),
        //StreamData1(StreamValueType<ProtoStream<BufReader<TcpStream>>>),
        None,
        //KeyValueCollection(Vec<(String, ValueType)>),
    }

    #[derive(PartialEq)]
    pub enum ValueType {
        String,
        SignedNumber64,
        UnSignedNumber64,
        UnSignedNumber32,
        SignedNumber16,
        UnSignedNumber16,
        U8Vec,
        None,
    }

    impl ValueType{
        pub fn parse(value_type: &ValueType, value: &[u8]) -> Value {
            match value_type {
                ValueType::String => {
                                            Value::String(String::from_utf8(value.to_vec()).unwrap())
                                        }
                
                ValueType::SignedNumber64 => {
                                            Value::SignedNumber64(i64::from_be_bytes(value.try_into().unwrap()))
                                        }

                ValueType::UnSignedNumber64 => {
                                            Value::UnSignedNumber64(u64::from_be_bytes(value.try_into().unwrap()))
                                        }   
                ValueType::UnSignedNumber32 => {
                    Value::UnSignedNumber32(u32::from_be_bytes(value.try_into().unwrap()))
                    }
                ValueType::SignedNumber16 => {
                    Value::SignedNumber16(i16::from_be_bytes(value.try_into().unwrap()))
                }
                ValueType::UnSignedNumber16 => {
                    Value::UnSignedNumber16(u16::from_be_bytes(value.try_into().unwrap()))
                },
                ValueType::U8Vec => {
                                            Value::U8Vec(value.to_vec())
                                        }
                ValueType::None => {
                                            Value::None
                                        }
            }
        }
    }

    #[allow(unused)]
        async fn write<W: AsyncWrite + Unpin>(value: &Value, mut writer: W) -> Result<(), ParserError> {
            match value {
                Value::String(s) => {
                                writer.write(s.as_bytes()).await?;
                            }
                Value::SignedNumber64(num) => {
                                writer.write_i64(*num).await?;
                            }
                Value::UnSignedNumber64(num) => {
                                writer.write_u64(*num).await?;
                            }
                Value::UnSignedNumber32(num) => {
                    writer.write_u32(*num).await?;
                }
                Value::U8Vec(data) => {
                                writer.write_all(&data[..]).await?;
                            }
                Value::SignedNumber16(num) => {
                    writer.write_i16(*num).await?;
                }
                Value::UnSignedNumber16(num) => {
                    writer.write_u16(*num).await?;
                },

                

                Value::None => todo!(),
            }
            Ok(())
        }
    

    impl Value {

        
        

        #[allow(unused)]
        async fn write<W: AsyncWrite + Unpin>(& self, mut writer: W) -> Result<(), ParserError> {
            match self {
                Value::String(s) => {
                                writer.write(s.as_bytes()).await?;
                            }
                Value::SignedNumber64(num) => {
                                writer.write_i64(*num).await?;
                            }
                Value::UnSignedNumber64(num) => {
                                writer.write_u64(*num).await?;
                            }
                Value::UnSignedNumber32(num) => {
                    writer.write_u32(*num).await?;
                }
                Value::U8Vec(data) => {
                                writer.write_all(&data[..]).await?;
                            }
                Value::SignedNumber16(num) => {
                    writer.write_i16(*num).await?;
                }
                Value::UnSignedNumber16(num) => {
                    writer.write_u16(*num).await?;
                },

                

                Value::None => todo!(),
            }
            Ok(())
        }
    }

    /*     #[allow(unused)]
    pub trait RequestProcessorRegistrar {
        fn register_request_processor<'a, H, RI>(
            request_type: String,
            request_info: RI,
            request_handler: H,
        ) where
            H: RequestHandler,
            RI: RequestInfo;
    } */

    

    /* pub trait RequestParse {
        #[allow(unused)]
        async fn parse_request<RI, RequestStream>(
            &self,
            reader: RequestStream,
            request_spec: &Placeholder,
        ) -> Result<RI, ParserError>
        where
            /*H: RequestHandler,*/
            RI: RequestInfo,
            RequestStream: AsyncRead + Unpin;
    } */

    /* impl RequestParse for Parser {
        async fn parse_request<RI, RequestStream, >(
            &self,
            reader: RequestStream,
        ) -> Result<RI, ParserError>
        where
            RI: RequestInfo,
            RequestStream: AsyncRead + Unpin,
        {


            todo!()
        }
    } */

    /*
    use tokio::stream::StreamExt;



    async fn process_stream(mut stream: impl Stream<Item = String>) {

    while let Some(value) = stream.next().await {

        println!("Value: {}", value);

    }

    }*/

    pub trait InfoProvider:  Send + Sync{
        #[allow(unused)]
        fn get_info(&self, key: &String) -> Option<&Value>;

        #[allow(unused)]
        fn get_info_mut(&mut self, key: &String) -> Option<&mut Value>;

        #[allow(unused)]
        fn get_keys_by_group_name(&self, name: String) -> Option<Vec<& String>>;

        fn add_info(&mut self, key: String, value: Value);

        //fn add_transient_info(&mut self, key: String, value: Value);

        fn has_all_data(&self) -> bool;

    }

    pub trait RequestInfo: InfoProvider {
        
    }

    pub trait ResponseInfo: InfoProvider {
    }

    #[allow(unused)]
    pub trait RequestFactory<REQI, REQSER, REQH, REQERRH, RESI> : Send + Sync
    where
        REQI: RequestInfo,
        REQSER: RequestSerializer<REQI>,
        REQH: RequestHandler<REQI, RESI>,
        REQERRH: RequestErrorHandler<REQI, RESI>,
        RESI: ResponseInfo,
    {
        fn get_request_spec(&self) -> &ListSpec;
        fn create_request_info(&self) -> REQI;
        fn create_request_serializer(&self) -> REQSER;
        fn create_request_handler(&self) -> REQH;
        fn create_error_request_handler(&self) -> REQERRH;
    }

    #[allow(unused)]
    pub trait ResponseFactory<RESI, RESS, RESH, RESERRH>: Send + Sync
    where
        RESI: ResponseInfo,
        RESS: ResponseSerializer<RESI>,
        RESH: ResponseHandler<RESI>,
        RESERRH: ResponseErrorHandler<RESI>,
    {
        fn get_response_spec(&self) -> &ListSpec;
        fn create_response_info(&self) -> RESI;
        fn create_response_serializer(&self) -> RESS;
        fn create_response_handler(&self) -> RESH;
        fn create_error_responset_handler(&self) -> RESERRH;
    }

    #[async_trait]
    pub trait RequestHandler<REQI, RESI> : Send + Sync
    where
        REQI: RequestInfo,
        RESI: ResponseInfo,
    {
        async fn handle_request(&self, request: &REQI, response: &mut RESI) -> Result<RESI, ParserError>;
    }

    pub trait ResponseHandler<RESI> : Send + Sync
    where
        RESI: ResponseInfo,
    {
        #[allow(unused)]
        fn handle_response(&self, response: &RESI) -> Result<(), ParserError>;
    }

    pub trait ResponseErrorHandler<RESI>  : Send + Sync
    where
        RESI: ResponseInfo,
    {
        #[allow(unused)]
        fn handle_response_error<E>(
            &self,
            response_info: &RESI,
            error: E,
        ) -> Result<(), ParserError>;
    }

    

    pub trait RequestErrorHandler<REQI, RESI>: Send + Sync
    where
        REQI: RequestInfo,
        RESI: ResponseInfo,
    {
        #[allow(unused)]
        fn handle_request_error<E>(&self, request: &REQI, error: E) -> Result<RESI, ParserError>;
    }

    #[async_trait]
    pub trait RequestSerializer<
        REQI: RequestInfo> : Send + Sync
    {
        #[allow(unused)]
        async fn serialize_to<W>(
            &self,
            req: &mut REQI,
            writer: W,
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        async fn deserialize_from<'a, B>(
            &self,
            request_info: &'a mut REQI,
            reader: B,
            spec: &dyn SpecDeserialize,
        ) -> Result<&'a mut REQI, ParserError> where B:AsyncRead + Unpin + Send + Sync;        
    }


    #[async_trait]
    pub trait ResponseSerializer<RSI>: Send + Sync 
    where RSI: ResponseInfo ,
        
    {
        async fn serialize_to<W>(
            &self,
            req: RSI,
            writer: W,
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError>
        where W: AsyncWrite + Unpin + Send + Sync;

        #[allow(unused)]
        async fn deserialize_from<'a, R>(&self,  
            response_info: &'a mut RSI,
            reader: &mut BufReader<R>,
            spec: &dyn SpecDeserialize) -> Result<&'a mut RSI, ParserError>
        where R:SpecRead;
    }

    #[allow(unused)]
    pub struct DefaultSerializer;

    #[async_trait]
    impl<REQI>
        RequestSerializer<REQI> for DefaultSerializer
        where 
            REQI: RequestInfo + 'static,
    {
        async fn serialize_to<W>(
            &self,
            request_info: &mut REQI,
            writer: W,
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError> 
        where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            spec.serialize(request_info,&mut protocol_writer).await?;
            /* protocol_writer
                .write_composite(spec, request_info, None)
                .await?; */
            Ok(())
        }

        async fn deserialize_from<'a, B>(
            &self,
            mut request_info:  &'a mut REQI,
            reader: B,
            spec: &dyn SpecDeserialize,
        )  -> Result<&'a mut REQI, ParserError> 
        where B:AsyncRead + Unpin + Send + Sync  {
            let mut protocol_reader = ProtocolBuffReader::new( BufReader::new(reader), 1024);
            let parse_result = spec.deserialize(request_info,&mut  protocol_reader).await;
            /* let result = protocol_reader
            .parse_composite(&mut request_info, spec).await; */
            
            if let Err(parser_error) = parse_result{
                if let ParserError::EndOfStream = parser_error  {
                    if request_info.has_all_data() {
                        return Ok(request_info);
                    }
                    return Err(ParserError::EndOfStream);
                } else {
                    return Err(parser_error);
                }
            }
            Ok(request_info)
        }        
    }

    

    

    #[async_trait]
    impl<RESI>
        ResponseSerializer<RESI> for DefaultSerializer
        where 
            RESI: ResponseInfo + 'static,
    {
        async fn serialize_to<W>(
            &self,
            response_info: RESI,
            writer: W,
            spec: &dyn SpecSerialize,
        ) -> Result<(), ParserError> where W: AsyncWrite + Unpin + Send + Sync {
            let mut protocol_writer = ProtocolBuffWriter::new(writer);
            spec.serialize(&response_info,&mut protocol_writer).await?;
            Ok(())
        }

        //(&self, mut response_info: RSI,reader: R, spec: &Placeholder)
        //async fn deserialize_from(&self,  response_info: &mut RSI,reader: &mut BufReader<&mut R>, spec: &Placeholder) -> Result<RSI, ParserError>;

        async fn deserialize_from<'a, R>(
            &self,
            response_info:&'a mut RESI,
            reader: &mut BufReader< R>,
            spec: &dyn SpecDeserialize,
        ) -> Result<&'a mut RESI, ParserError> 
        where R:SpecRead {
            let mut protocol_reader = ProtocolBuffReader::new(reader, 1024);
            let parse_result = spec.deserialize(response_info,&mut  protocol_reader).await;
            /* let result = protocol_reader
            .parse_composite(&mut request_info, spec).await; */
            
            if let Err(parser_error) = parse_result{
                if let ParserError::EndOfStream = parser_error  {
                    if response_info.has_all_data() {
                        return Ok(response_info);
                    }
                    return Err(ParserError::EndOfStream);
                } else {
                    return Err(parser_error);
                }
            }
            Ok(response_info)
            /* protocol_reader
                .parse_composite(response_info, spec).await?; */
                
            //Ok(())
        }        
    }

    #[allow(unused)]
    pub struct Protocol {
        name: ProtocolVersion,
        transport: Transport,
        format: ProtocolFormat,
        request_place_holder: ListSpec, //Placeholder,
        response_place_holder: ListSpec,
    }

    #[allow(unused)]
    pub enum Transport {
        UDP,
        TCP,
    }

    #[allow(unused)]
    pub enum ProtocolFormat {
        Text,
        Binary,
    }

    #[allow(unused)]
    pub struct ProtocolBuilder<RQI, RSI>
    where
        RQI: RequestInfo,
        RSI: ResponseInfo,
    {
        name: Option<String>,
        version: Option<String>,
        transport: Option<Transport>,
        format: Option<ProtocolFormat>,
        request_info: Option<RQI>,
        response_info: Option<RSI>,
    }

    #[derive(Debug,)]
    #[allow(unused)]
    pub enum ServerError {
        StartError(String),
        StopError,
        RequestError(ParserError),
        ResponseError(ParserError),
        IOError(std::io::Error),
    }
    #[async_trait]
    pub trait Server {
        #[allow(unused)]
        async fn start(&'static mut self) -> Result<(), ServerError>;

        #[allow(unused)]
        async fn stop(&mut self) -> Result<(), ServerError>;

        /* async fn handle_request(&self, request: RQI) -> Result<RSI, ServerError>;
        async fn send_response(&self, response: RSI) -> Result<(), ServerError>; */
    }

    impl From<std::io::Error> for ServerError {
        fn from(error: std::io::Error) -> Self {
            ServerError::IOError(error)
        }
    }

    pub trait ProtocolConfig: Send + Sync
    {
        /* type R: AsyncRead + Unpin + Send + Sync;
        type W: AsyncWrite + Unpin + Send + Sync; */
        type REQF: RequestFactory<Self::REQI, Self::REQSER, Self::REQH, Self::REQERRH, Self::RESI>;
        type RESF: ResponseFactory<Self::RESI, Self::RESSER, Self::RESH, Self::RESERRH>;
        type REQI: RequestInfo;
        type RESI: ResponseInfo;
        type REQSER: RequestSerializer<Self::REQI>;
        type RESSER: ResponseSerializer<Self::RESI>;

        type REQH: RequestHandler<Self::REQI, Self::RESI>;
        type RESH: ResponseHandler<Self::RESI>;
        type REQERRH: RequestErrorHandler<Self::REQI, Self::RESI>;
        type RESERRH: ResponseErrorHandler<Self::RESI>;
    }
    
    #[derive(Builder)]
    #[builder(pattern = "owned")]
    pub struct ServerInstance<CFG> 
    where CFG: ProtocolConfig{
        hosts: Vec<String>,

        /* #[allow(unused)]
        protocol: Protocol, */

        request_factory: CFG::REQF,
        #[allow(unused)]
        response_factory: CFG::RESF,

        #[builder(setter(skip))]
        listeners: Vec<TcpListener>,

       // phantom_req_info: std::marker::PhantomData<CFG::REQF>,
       // phantom_res_info: std::marker::PhantomData<CFG::RESF>,
    }

    

    
    impl <CFG> ServerInstance<CFG> 
    where CFG: ProtocolConfig,
                 
    {
        #[allow(unused)]
        async fn create_listeners(&mut self) -> Result<(), ServerError> {
            for host in &self.hosts {
                let option = host.split_once(":");
                match option {
                    Some((host, port)) => {
                        let listener = TcpListener::bind(format!("{}:{}", host, port))
                            .await
                            .unwrap();
                        self.listeners.push(listener);
                    }
                    None => {
                        return Err(ServerError::StartError(
                            "Invalid host:port format".to_owned(),
                        ));
                    }
                }
            }
            Ok(())
        }

        async fn accept_connections(&'static self, tcp_listener: &'static TcpListener) {
            tokio::spawn(async move {
                loop {
                    let (socket, addr) = tcp_listener.accept().await.unwrap();
                    println!("Accepted connection from {}", addr);

                    let _handle = tokio::spawn(async move {
                        self.handle_connection(socket).await;
                    });
                }
            });
        }

        async fn handle_connection(&'static self, mut socket: TcpStream) {
            let mut req_info = self.request_factory.create_request_info();
            let serializer = self.request_factory.create_request_serializer();

            let mut res_info = self.response_factory.create_response_info();
            //let read = Box::new(&mut socket);
            //let read = & mut socket;
            
           // let x = *read;
            let mut buf_reader  = BufReader::new(&mut socket);  
            //serializer.serialize_to(req_info, socket, self.request_factory.get_request_spec(),).await;
             let request_info = 
             serializer
                .deserialize_from(
                    &mut req_info,
                    &mut buf_reader,
                    self.request_factory.get_request_spec(),
                )
                .await
                .unwrap(); 
            let result = CFG::REQH::handle_request(
                &self.request_factory.create_request_handler(),
                &request_info,
                &mut res_info
            ).await;
            match result {
                Ok(response_info) => {
                    let serializer = self.response_factory.create_response_serializer();
                     serializer
                        .serialize_to(
                            response_info,
                            socket,
                            self.response_factory.get_response_spec(),
                        )
                        .await
                        .unwrap(); 
                }
                Err(e) => {
                    println!("Error handling request: {:?}", e);
                }
            } 
        }
    }

    #[async_trait]
    impl<CFG> Server for ServerInstance<CFG> 
    where CFG: ProtocolConfig{
        async fn start(&'static mut self) -> Result<(), ServerError> {
            //let host = String::new();
            self.create_listeners().await?;
            
            for listener in &self.listeners {
                let _result = self.accept_connections(listener).await;
                println!("hh{:?}", listener);
            }

            Ok(())
        }

        async fn stop(&mut self) -> Result<(), ServerError> {
            todo!()
        }

        /* async fn handle_request(&self, request: RequestInfo) -> Result<ResponseInfo, ParserError> {
            todo!()
        }

        async fn send_response(&self, response: ResponseInfo) -> Result<(), ParserError> {
            todo!()
        } */
    }

    /* impl<RQI, RSI> ProtocolBuilder<RQI, RSI>
    where
        RQI: RequestInfo,
        RSI: ResponseInfo,
    {
        pub fn new(
            name: String,
            version: Option<String>,
            transport: Transport,
            format: ProtocolFormat,
        ) -> Self {
            ProtocolBuilder {
                name,
                version,
                transport,
                format,
                request_info: RequestInfo::default(),
                response_info: ResponseInfo::default(),
            }
        }
    } */

    #[allow(unused)]
    pub trait Processor {
        fn process(_req: Request, _res: Response) {
        }
    }

    #[allow(unused)]
    pub struct ProtocolVersion {
        name: String,
        version: Option<String>,
    }

    pub struct Request {}

    pub struct Response {}

    pub enum HeaderValue{
        String,
        
    }

    /* #[derive(Default)]
    #[allow(unused)]
    pub enum PlaceHolderType {
        #[default]
        AnyString,
        ExactString(String),
        OneOf(Vec<String>),
        BytesOfSizeFromHeader(String),
        Bytes,
        BytesOfSizeN(u32),
        Space,
        NewLine,
        Delimiter(String),
        Composite,
        RepeatMany,
        RepeatN(u8),
        StreamValue(String),
    } */

    /* #[allow(unused)]
    struct Key(String); */

    #[allow(unused)]
    pub enum PlaceHolderIdentifier {
        #[allow(unused)]
        Name(String),
        #[allow(unused)]
        Key,
        InlineKeyWithValue(String),
        Value,
    }

    impl Default for PlaceHolderIdentifier {
        fn default() -> Self {
            PlaceHolderIdentifier::Name(String::new())
        }
    }

    impl Debug for PlaceHolderIdentifier{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(arg0) => f.debug_tuple("Name").field(arg0).finish(),
            Self::Key => write!(f, "Key"),
            Self::InlineKeyWithValue(arg0) => f.debug_tuple("InlineKeyWithValue").field(arg0).finish(),
            Self::Value => write!(f, "Value"),
        }
    }
    }

    fn is_send<T:Send>(t: T){}

    /* fn test(){
        is_send(Placeholder::default());
        //is_send(Req::default());
        //is_send(Rc::new("test".to_owned()));
    } */

    /* #[derive(Default)]
    pub struct Placeholder<T> where T: Parse + Send + Sync {
        #[allow(dead_code)]
        name: String,
        parser: T,
        //pub constituents: Option<Vec<Box<dyn Parse + Send + Sync>>>,

        #[allow(dead_code)]
        pub optional: bool,
    } */

    /* impl <T:Parse  +Send + Sync> Placeholder<T> {
        pub fn new(
            name: String,            //constituents: Option<Vec<Placeholder>>,
            parse: T,
            optional: bool,
        ) -> Self {
            Placeholder {
                name,
                parse,                
                optional,
            }
        }

        #[allow(unused)]
        pub fn new_key_placeholder(
            name: String,
            parse: T,
            optional: bool,
        ) -> Placeholder<Key<T>> {
            Placeholder::<Key<T>> {
                name,
                optional,
                parse:Key(parse),
                
            }
        }

        #[allow(unused)]
        pub fn new_placeholder_with_key(
            name: String,
            constituents: Option<Vec<Placeholder>>,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name,
                place_holder_type,                
                optional,
            }
        }

        #[allow(unused)]
        pub fn new_value_placeholder(
            name: String,
            place_holder_type: PlaceHolderType,
            optional: bool,
        ) -> Self {
            Placeholder {
                name,
                place_holder_type,
                optional,
            }
        }
    } */

    /* pub enum PlaceHolderValue {
        #[allow(unused)]
        AnyString(String),
        #[allow(unused)]
        OneOf(String),
        #[allow(unused)]
        Delimiter(String),
        #[allow(unused)]
        AnyBytes(Vec<u8>),
    } */

    #[allow(dead_code)]
    trait TokenParser {
        async fn read_string(until_delimiter: String) -> String;
    }
    pub trait SpecRead: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Sync {
    }

    pub trait SpecWrite: PlaceHolderWrite + AsyncWrite + Unpin + Send + Sync {
    }

    #[async_trait]
    pub trait SpecDeserialize: Send + Sync/* : Spec */{
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>;
        
    }

    enum SpecType{
        Composite(String),
        RepeatMany(String),
        RepeatN(String, u16),
        Key(String),
        Value(String),
        Simple(String),
    }

    struct SpecContext{
        contexts: Vec<SpecType>,
    }
    /*
        request_line.protocol_version
        request_line.protocol_uri

        headers.hello.1
        headers.hello.2
        headers.hello.3
     */

    impl SpecContext{
        fn new()->Self{
            Self { contexts: vec!() }
        }
    }

    #[async_trait]
    pub trait  SpecSerialize: Send + Sync/* :Spec */{
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>;
        
    }

    #[async_trait]
    impl SpecSerialize for InlineKeyWithValue{
        
        async fn serialize (
            &self,
            info_provider: &( dyn InfoProvider + Send + Sync ),
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>{
            let name = self.get_meta_data().get_name();
            let value = info_provider.get_info(name);
            if let Some(value) = value{
                write(value, writer).await?;
                Ok(())
            }else if !self.2.optional {
                return Err(ParserError::MissingData(name.to_owned()));
            }else{
                Ok(())
            }
        }
    }
    

    trait UndoableParse{

        async fn undoable_parse (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>;

    }

    impl <P> UndoableParse for P where P:SpecDeserialize{
        async fn undoable_parse (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let marker = reader.mark();
                let result = self.deserialize(info_provider, reader).await;
                match result {
                    Ok(value_type) => {
                        reader.unmark(&marker);
                        return Ok(value_type);                    
                    }
                    Err(e) => {
                        reader.reset(&marker)?;
                        return Err(e);
                    }
                }
        }
    }

    impl UndoableParse for dyn Spec{
        async fn undoable_parse (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            deserialize_spec(self, info_provider, reader).await
        }
    }

    impl UndoableParse for dyn StringSpec{
        async fn undoable_parse (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            deserialize_spec(self, info_provider, reader).await
        }
    }

    async fn deserialize_spec<P: Spec + ?Sized>(spec:&P, info_provider: &mut (dyn InfoProvider + Send + Sync), reader: &mut dyn SpecRead) -> Result<Value, ParserError> {
        let marker = reader.mark();
        let result = spec.deserialize(info_provider, reader).await;
        Ok(match result {
            Ok(value_type) => {
                reader.unmark(&marker);
                return Ok(value_type);                    
            }
            Err(e) => {
                reader.reset(&marker)?;
                return Err(e);
            }
        })
    }


    /* impl <P> RollabackableParse for P where P: Parse{

    } */

    /* #[async_trait]
    pub trait MarkAndResetParse {

        async fn mark(reader: &mut (dyn SpecRead))-> Marker{
            reader.mark()
        }

        async fn reset(reader: &mut (dyn SpecRead), marker: Marker)->Result<(), ParserError>{
            reader.reset(marker)?;
            Ok(())
        }

        async fn clear_mark(reader: &mut (dyn SpecRead), marker: Marker){
            reader.unmark(marker);
        }

        async fn mark_and_parse(
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
            
        )  -> Result<crate::core::Value, ParserError>;/*{
                let marker = reader.mark();
                let result = self.parse(info_provider, reader).await;
                match result {
                    Ok(value_type) => {
                        reader.unmark(&marker);
                        return Ok(value_type);                    
                    }
                    Err(e) => {
                        reader.reset(&marker)?;
                        return Err(e);
                    }
                }
            } */
    } */

    /* #[async_trait]
    impl<P> MarkAndResetParse for P where P: Parse {
        async fn mark_and_parse(
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
            
        )  -> Result<crate::core::Value, ParserError>{
                let marker = reader.mark();
                let result = self.parse(info_provider, reader).await;
                match result {
                    Ok(value_type) => {
                        reader.unmark(&marker);
                        return Ok(value_type);                    
                    }
                    Err(e) => {
                        reader.reset(&marker)?;
                        return Err(e);
                    }
                }
            }
        
    } */

    //impl MarkAndResetParse for dyn StringSpec{}// where P: Parse {}

    //impl Parse for dyn StringSpec{}// where P: Parse {}

    //impl MarkAndResetParse for dyn Spec{}// where P: Parse {} Box<dyn Spec>


    

     /* #[async_trait]
    impl Parse for dyn StringSpec{
        async fn parse (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>{
            Ok(Value::None)
        }
    } */

    /* #[async_trait]
    impl SpecDeserialize for dyn Spec{
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>{
            Ok(Value::None)
        }
    }  */

    /* impl MarkAndResetParse for dyn SerializeDeserialize {}

    impl MarkAndResetParse for dyn Parse + Send + Sync {} */

    //impl MarkAndResetParse for dyn Spec + Send + Sync {}

    #[derive(Debug, Clone)]
    pub enum Separator{
        Delimiter(String),
        NBytes(u32),
        EndOfStream,
    }

    impl Default for Separator {
        fn default() -> Self {
            Separator::EndOfStream
        }
    }

    #[derive( PartialEq)]
    pub struct SpecMetaData{
        name: String,
        value_type: ValueType,
        optional: bool,
    }

    impl Default for SpecMetaData{
        fn default() -> Self {
            SpecMetaData {
                name: String::new(),
                value_type: ValueType::None,
                optional: false,
            }
        }
    }

    impl  SpecMetaData{
        pub fn new(name: String, value_type: ValueType, optional: bool) -> Self {
            SpecMetaData {
                name,
                value_type,
                optional,
            }
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }
        pub fn get_value_type(&self) -> &ValueType {
            &self.value_type
        }
        pub fn is_optional(&self) -> bool {
            self.optional
        }
    }

    pub trait DelimitedSpec: Spec + Default{
        fn set_delimiter(&mut self, delimiter: Separator) ;
    }

    pub trait StringSpec: Spec + Send + Sync{}

    #[derive(Default)]
    pub struct DelimitedStringSpec{
        spec_meta_data: SpecMetaData,
        until: Separator,
    }

    impl Spec for DelimitedStringSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    impl  StringSpec for DelimitedStringSpec{}

    impl  DelimitedSpec for DelimitedStringSpec{
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.until = delimiter;
        }
    }
    

    /* trait PlaceHolderType: Default + Send + Sync{}

    #[derive(Default)]
    struct KeyItem(String);

    #[derive(Default)]
    struct ValueItem(String);

    #[derive(Default)]
    struct InlineKeyItem(String);
    
    #[derive(Default)]
    struct DontCare();

    #[derive(Default)]
    struct Composite();

    impl PlaceHolderType for KeyItem{}
    impl PlaceHolderType for ValueItem{}
    impl PlaceHolderType for InlineKeyItem{}
    impl PlaceHolderType for DontCare{}
    impl PlaceHolderType for Composite{} */

/*     #[derive(PartialEq)]
    enum PlaceHolderType{
        DontCare, Composite, KeyItem(String), ValueItem(String), InlineKeyItem(String)
    } */

    

    enum RepeatCount{
        Fixed(u32),
        Delimited(Separator),
    }

    impl Default for RepeatCount{
        fn default() -> Self {
            RepeatCount::Fixed(2)
        }
    }

    #[derive(Default)]
    struct RepeatManySpec{
        spec_meta_data: SpecMetaData,        
        repeat_count: RepeatCount,
        constituents: ListSpec,
    }

    impl DelimitedSpec for RepeatManySpec{
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.repeat_count = RepeatCount::Delimited(delimiter);
        }
    }

    impl Spec for RepeatManySpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    

    #[async_trait]
    impl SpecDeserialize for RepeatManySpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError> {
            // Implementation for parsing repeat many spec
            let mut repeat_count = 0;
            loop{
                self.constituents.undoable_parse(info_provider, reader).await?;
                repeat_count += 1;
                if let RepeatCount::Fixed(count) = &self.repeat_count {
                    if repeat_count >= *count {
                        break;
                    }
                } else if let RepeatCount::Delimited(ref delimiter) = &self.repeat_count {
                    let result = DelimitedStringSpec::new("".to_owned(), delimiter.clone(), false).undoable_parse(info_provider, reader).await;
                    //let next_value = reader.read_placeholder_until(delimiter.to_owned(), self.get_meta_data()).await?;
                    if result.is_ok(){
                        break;
                    }
                }

            }

            Ok(Value::None) // Return appropriate value based on parsing
            
        }
    }

    #[async_trait]
    impl SpecSerialize for RepeatManySpec
    {
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ),
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>
        {
            /* let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.get_meta_data().is_optional(), writer).await?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            } */
           //TODO:Complete implementation
            Ok(())
        }
    }


    pub trait Spec: SpecSerialize + SpecDeserialize + Send + Sync  {
        fn get_meta_data(&self) -> &SpecMetaData;
    }

 /*    trait SimpleValueSpecBuilder{



    } */




    impl DelimitedStringSpec{
        fn new(name: String, delimiter: Separator,  optional: bool) -> Self {
            DelimitedStringSpec {                
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                until: delimiter,
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for DelimitedStringSpec
    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError>      
        {
            //let mut buf = vec![];
             let value = match self.until {
                Separator::Delimiter(ref delimiter) => {
                    reader.read_placeholder_until(delimiter.to_owned()).await?
                }
                Separator::NBytes(size) => {
                    reader.read_bytes( ReadBytesSize::Fixed(size)).await?
                }
                Separator::EndOfStream => {
                    reader.read_bytes(ReadBytesSize::Full).await?
                }
            };

            if let Some(value) = value {
                info_provider.add_info(self.spec_meta_data.name.clone(), Value::U8Vec(value.clone()));
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read value for placeholder: {:?}",
                    self.get_meta_data().name
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for DelimitedStringSpec
    {
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ),
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.get_meta_data().is_optional(), writer).await?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            }
            Ok(())
        }
    }

    struct ExactStringSpec{
        input: String,
        spec_meta_data: SpecMetaData,
    }

    impl StringSpec for ExactStringSpec {}

    impl StringSpec for OneOfSpec {}

    impl ExactStringSpec{
        fn new(name: String, input: String, optional: bool) -> Self {
            ExactStringSpec {
                input,
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
            }
        }
    }

    impl  Spec for ExactStringSpec {
        
        fn get_meta_data(&self)-> &SpecMetaData{
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl SpecDeserialize for ExactStringSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut dyn SpecRead,
        ) -> Result<Value, ParserError>
        {
            let value = reader.read_placeholder_as_string(self.input.clone()).await?;
            if let Some(value) = value {
                info_provider.add_info(self.spec_meta_data.name.clone(), Value::U8Vec(value.clone()));
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &value));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read exact string for placeholder: {:?}",
                    self.get_meta_data().get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for ExactStringSpec
    {
        async fn serialize (
            &self,
            info_provider: & ( dyn InfoProvider + Send + Sync ),
            writer: &mut (dyn SpecWrite),
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.get_meta_data().is_optional(), writer).await?;
            Ok(())
        }
    }

    struct KeyValueSpec{
            spec_metadata: SpecMetaData,
            key: Key,
            value: ValueSpec,
        }
    
    impl KeyValueSpec{
        pub fn new(key: Key, value: ValueSpec, spec_metadata: SpecMetaData) -> Self {
            KeyValueSpec {
                spec_metadata,
                key,
                value,
            }
        }
    }

    impl Spec for KeyValueSpec{
        fn get_meta_data(&self) -> &SpecMetaData{
            &self.spec_metadata
        }
    }

    #[async_trait]
    impl SpecSerialize for KeyValueSpec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.key.get_meta_data().get_name();
            self.key.serialize(info_provider, writer).await?;
            /* let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.spec_metadata.optional, writer).await?; */
            Ok(())
        }
    }

    #[async_trait]
    impl SpecDeserialize for KeyValueSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
            
        ) -> Result<Value, ParserError>
        {
            let key_name = self.key.undoable_parse(info_provider, reader).await?;
            if let Value::String(key) = key_name {
                let value = self.value.undoable_parse(info_provider, reader).await?;
                info_provider.add_info(key, value);
                return Ok(Value::None);
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read key-value pair for placeholder: {:?}",
                    self.get_meta_data().get_name()
                )))
            }
        }
    }

    pub struct NBytesSpec{
        spec_meta_data: SpecMetaData,
        size: u32,
    }

    impl  NBytesSpec{
        pub fn new(name: String, size: u32, optional: bool) -> Self {
            NBytesSpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::U8Vec, optional),
                size,
            }
        }
    }

    impl Spec for NBytesSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl SpecDeserialize for NBytesSpec{
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(self.size)).await?;
            if let Some(bytes) = bytes {
                info_provider.add_info(self.spec_meta_data.name.clone(), Value::U8Vec(bytes.clone()));
                return Ok(ValueType::parse(&self.get_meta_data().value_type, &bytes));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read {} bytes for placeholder: {:?}",
                    self.size, self.get_meta_data().get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for NBytesSpec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.get_meta_data().is_optional(), writer).await?;
            Ok(())
        }
    }

    

    struct AllBytesSpec{     
        spec_meta_data: SpecMetaData,           
    }

    impl Spec for AllBytesSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    #[async_trait]
    impl SpecDeserialize for AllBytesSpec{    
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            let bytes = reader.read_bytes(ReadBytesSize::Full).await?;
            if let Some(bytes) = bytes {
                info_provider.add_info(self.spec_meta_data.name.clone(), Value::U8Vec(bytes.clone()));
                return Ok(ValueType::parse(&self.get_meta_data().get_value_type(), &bytes));
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read {} bytes for placeholder: {:?}",
                    "remaining ", self.get_meta_data().name
                )))
            }
        }
    }

    #[async_trait]
    impl SpecSerialize for AllBytesSpec{
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.get_meta_data().is_optional(), writer).await?;
            Ok(())
        }
    }

    

/*     struct Spec<T:Parse + Send + Sync>{
        name: String,
        spec: T,
    } */



 /*    macro_rules! create_composite_parser_fields {
        ([$($args:ident),*]) => {
            paste!  {
                $( [<$args:lower>]:$args ),*
            }
        };
    } */

    macro_rules! create_composite_parser {
        
        ([$($args:ident),*], $arg_size:expr) => {
            paste::paste! {
                
                struct [< CompositeParser $arg_size>]<$($args),*> where
                    $($args: Parse + Send + Sync),* {
                        //create_composite_parser_fields!([$($args),*]),
                        $( [<$args:lower>]:$args ),*
                }


                #[async_trait]
                impl <$($args),*> Parse for [< CompositeParser $arg_size>]<$($args),*> where
                $($args: Parse + Send + Sync),*  {
                    async fn parse<IP, B>(
                        &self,
                        info_provider: &mut IP,
                        reader: &mut B,
                        spec: &Placeholder,
                    ) -> Result<ValueType, ParserError>
                    where
                        IP: InfoProvider + Send + Sync,
                        B: PlaceHolderRead + MarkAndRead + AsyncRead + Unpin + Send + Sync,
                    {
                        paste::paste! {
                            $(self.[<$args:lower>].mark_and_parse(info_provider, reader, spec).await?;)*
                            Ok(ValueType::None) // or some other appropriate return value
                        }
                    }
                }
            }
        }
    }


/*     create_composite_parser!([T1], 1);
    create_composite_parser!([T1, T2], 2);
    create_composite_parser!([T1, T2, T3], 3);
    create_composite_parser!([T1, T2, T3, T4], 4);
    create_composite_parser!([T1, T2, T3, T4, T5], 5); */


    #[derive(Default)]
    pub struct OneOfSpec{
        spec_meta_data: SpecMetaData,
        values: Vec<String>,        
        until: Separator,
    }

    

    impl DelimitedSpec for OneOfSpec{
        fn set_delimiter(&mut self, delimiter: Separator)  {
            self.until = delimiter;
        }
    }

    impl Spec for OneOfSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }

    impl OneOfSpec{
        pub fn new(name: String, optional: bool, values: Vec<String>) -> Self {
            OneOfSpec {
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional),
                values,
                until: Separator::EndOfStream,
            }
        }

        pub fn add_value(&mut self, value: String) {
            self.values.push(value);
        }

        pub fn set_delimiter(&mut self, delimiter: Separator) {
            self.until = delimiter;
        }

        pub fn get_values(&self) -> &Vec<String> {
            &self.values
        }
    }

    #[async_trait]
    impl SpecDeserialize for OneOfSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>
        {
            let result = DelimitedStringSpec::new(
                format!("{}-expect-one-of", self.get_meta_data().get_name()),
                self.until.clone(),
                self.get_meta_data().is_optional())
                .undoable_parse(info_provider, reader).await?;
            if let Some(value) = &result.get_string_value() {
                if self.values.contains(value) {
                info_provider.add_info(self.spec_meta_data.name.clone(), Value::String(value.clone()));
                return Ok(result);
                } else {
                    return Err(ParserError::MissingValue(format!(
                        "Expected one of {:?}, but got: {:?}",
                        self.values, value
                    )));
                }
            } else {
                Err(ParserError::MissingValue(format!(
                    "Expected one of {:?}, but got: {:?}",
                    self.values, result
                )))
            }
        }
    }


    #[async_trait]
    impl SpecSerialize for OneOfSpec{
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.get_meta_data().get_name();            
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.get_meta_data().is_optional(), writer).await?;
            if let Separator::Delimiter(delimiter) = &self.until{
                writer.write(delimiter.as_bytes()).await?;
            }
            Ok(())
        }
    }

    trait SpecSerde: SpecSerialize + SpecDeserialize {}
    //trait SpecSerde

    #[derive(Default)]
    pub struct ListSpec{            
        spec_meta_data: SpecMetaData,
        constituents: Vec<Box< (dyn Spec)>>,  
        //phantom: PhantomData<T>,
    }

    

    #[async_trait]
    impl SpecDeserialize for ListSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            for constituent in &self.constituents {                
                constituent.undoable_parse(info_provider, reader).await?;
            }
            Ok(Value::None) // or some other appropriate return value
        }
    }

   /*  #[async_trait]
    trait CompositeSerializer {
        async fn write_to( &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>;
    } */

    #[async_trait]
    impl SpecSerialize for ListSpec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            for constituent in &self.constituents {                
                constituent.serialize(info_provider, writer).await?;
            }
            Ok(()) // or some other appropriate return value
        }
    }

    /* impl Spec for Box<dyn Spec>{
        fn get_meta_data(&self) -> &SpecMetaData {
            self.get_meta_data()
        }
    }

    #[async_trait]    
    impl SpecSerializer for Box<dyn Spec>{
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            Ok(())
        }
    } */

    impl ListSpec {
        pub fn new(name: String, value_type: ValueType, optional: bool) -> Self {
            ListSpec {
                spec_meta_data: SpecMetaData::new(name, value_type, optional),
                constituents: Vec::new(),
            }
        }

        pub fn add_spec(&mut self, constituent: Box<dyn Spec> ) {
            self.constituents.push(constituent);
        }
    }

    impl Spec for ListSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_meta_data
        }
    }
    

    #[derive(Default)]
    struct ValueSpec(Box<dyn Spec>, SpecMetaData);

    impl Default for Box<dyn Spec> {
        fn default() -> Self {
            Box::new(DelimitedStringSpec::default())
        }
    }

    impl Spec for ValueSpec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }

    

    struct InlineKeyWithValue(Box<dyn Spec>, String, SpecMetaData);



    impl Spec for InlineKeyWithValue {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.2
        }
    }
    
    
    #[derive(Default)]
    struct Key(Box<dyn StringSpec>, SpecMetaData) ;

    impl Spec for Key {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.1
        }
    }

    #[async_trait]
    impl SpecSerialize for Key {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.1.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.1.optional, writer).await?;
            Ok(())
        }
    }

    impl Default for Box<dyn StringSpec> {
        fn default() -> Self {
            Box::new(DelimitedStringSpec::default())
        }
    }

    

    

    /* impl <P: Parse + Send + Sync> Parent<P> for Key<P>{
        fn add_child(&mut self, s: P) {
            self.0 = s;
        }
    } */

    #[async_trait]
    impl SpecDeserialize for Key

    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            self.0.undoable_parse(info_provider, reader).await
            /* let mut_ref = self.0;
            mut_ref.mark_and_parse(info_provider, reader).await */
        }
    }

    #[async_trait]
    impl SpecDeserialize for InlineKeyWithValue
    {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            self.0.undoable_parse(info_provider, reader).await.map(|value| {
                info_provider.add_info(self.1.clone(), value);
                Value::None // or some other appropriate return value
            })
        }
    }

    #[async_trait]
    impl SpecDeserialize for ValueSpec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),            
        ) -> Result<Value, ParserError>
        {
            self.0.undoable_parse(info_provider, reader).await
        }
    }

    #[async_trait]
    impl SpecSerialize for ValueSpec {
        async fn serialize(
            &self,
            info_provider: & (dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.1.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.1.optional, writer).await?;
            Ok(())
        }
    }

    async fn write_data(name: String, value:Option<&Value>, optional:bool, writer: &mut (dyn SpecWrite)) -> Result<(), ParserError>{
        if let Some(value) = value{
                write(value, writer).await?;
                Ok(())
            }else if !optional {
                return Err(ParserError::MissingData(name));
            }else{
                Ok(())
            }
    }

    #[derive(Default)]
    struct NumberU64Spec(SpecMetaData) ;

    #[derive(Default)]
    struct NumberI64Spec(SpecMetaData) ;

    #[derive(Default)]
    struct NumberU32Spec(SpecMetaData) ;

    #[derive(Default)]
    struct NumberU16Spec(SpecMetaData) ;

    trait NumberSpec: Send + Sync{}

    impl NumberSpec for NumberU64Spec{}
    impl NumberSpec for NumberU32Spec{}
    impl NumberSpec for NumberU16Spec{}
    impl NumberSpec for NumberI16Spec{}
    impl NumberSpec for NumberI64Spec{}

    impl Spec for NumberU64Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    impl Spec for NumberU16Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    impl Spec for NumberU32Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    
    impl Spec for NumberI16Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    impl Spec for NumberI64Spec {
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.0
        }
    }

    
    #[derive(Default)]
    struct NumberI16Spec(SpecMetaData);
    
    
    #[async_trait]
    impl SpecDeserialize for NumberU64Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber64, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberI64Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(8)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::SignedNumber64, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberU32Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber32, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberU16Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::UnSignedNumber16, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }

    #[async_trait]
    impl SpecDeserialize for NumberI16Spec {
        async fn deserialize(
            &self,
            info_provider: &mut (dyn InfoProvider + Send + Sync),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError> {
            let bytes = reader.read_bytes(ReadBytesSize::Fixed(4)).await?;
            if let Some(bytes) = bytes {
                Ok(ValueType::parse(&ValueType::SignedNumber16, &bytes))
            } else {
                Err(ParserError::MissingValue(format!(
                    "Unable to read 8 bytes for placeholder: {:?}",
                    self.0.get_name()
                )))
            }
        }
    }


    /* #[async_trait]
    impl SpecSerialize for dyn Spec {


        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    } */

    #[async_trait]
    impl SpecSerialize for NumberU16Spec {


        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberU32Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberU64Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberI64Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    #[async_trait]
    impl SpecSerialize for NumberI16Spec {
        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            let name = self.0.get_name();
            let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.0.optional, writer).await?;
            Ok(())
        }
    }

    pub trait ByteSpecGenerator{
        fn get_bytes_spec_of_size(&mut self, name: String, size:u32, optional: bool) -> NBytesSpec{
            NBytesSpec::new(name, size, optional)
        }
    }

    trait BuilderState:Default{}

    trait BuildGenericString:BuilderState{}

    trait BuildKeyString:BuilderState{}
    

    // Builder States

    #[derive(Default)]
    pub struct BuildFromScratch{}

    #[derive(Default)]
    pub struct BuildKey{
        key_spec_metadata: SpecMetaData,
    }

    #[derive(Default)]
    pub struct BuildKeyAvailable{
        key: Key,
    }

    #[derive(Default)]
    pub struct BuildValue{
        key: Key,
        value_spec_metadata: SpecMetaData,
    }

    #[derive(Default)]
    pub struct BuildInlineValue{
        key_name: String,
        value_spec_metadata: SpecMetaData,
    }
    
    #[derive(Default)]
    pub struct BuildDelimiter<D:DelimitedSpec, B:BuilderState>{
        delimiter_spec: D,
        parent_builder_state: B,
    }

    //Builder State implementations

    impl  BuilderState for BuildFromScratch{}
    impl  BuildGenericString for BuildFromScratch{}
    impl  BuilderState for BuildKey{}
    impl  BuildKeyString for BuildKey{}
    impl  BuilderState for BuildValue{}
    impl  BuilderState for BuildKeyAvailable{}
    impl  BuilderState for BuildInlineValue{}

    impl <D, B> BuilderState for BuildDelimiter<D, B> where D: DelimitedSpec, B:BuilderState{}

    
    //Proto Spec Builder trait
    pub trait ProtoSpecBuilder<S:BuilderState>: Default{
        fn build(self) -> ListSpec;
        fn add_spec(&mut self, spec: Box<dyn Spec>);
        fn replace_current_state_with_default(&mut self)-> S;
        fn get_state(&mut self)-> &mut S;
        fn set_state(&mut self, s: S);
        fn set_spec(&mut self, composite_spec:ListSpec);
        fn wrap_with_data<D>(self, data:D)->BuilderWrapperWithData<Self, D, S>;
        fn wrap(self) -> BuilderWrapper<Self, S>;
        fn replace_current_spec_with_default(&mut self) -> ListSpec;
    }

    // Struct implementing Spec Builder 
    #[derive(Default)]
    pub struct ProtoSpecBuilderData<S:BuilderState>{
        composite_spec: ListSpec,
        state: S,
    }

    impl <S> ProtoSpecBuilder<S> for ProtoSpecBuilderData<S> where S:BuilderState {
        fn build(self) -> ListSpec {
            self.composite_spec
        }

        fn add_spec(&mut self, spec: Box<dyn Spec>) {
            self.composite_spec.add_spec(spec);
        }
        
        fn replace_current_state_with_default(&mut self)-> S {
            mem::take(&mut self.state)   
        }
        
        fn set_spec(&mut self, composite_spec:ListSpec) {
            self.composite_spec = composite_spec;
        }

        fn replace_current_spec_with_default(&mut self) -> ListSpec{
            mem::take(&mut self.composite_spec)
        }
        
        fn get_state(&mut self)-> &mut S {
            &mut self.state
        }
        
        fn set_state(&mut self, s: S) {
            self.state = s;
        }

        fn wrap_with_data<D>(self, data:D)->BuilderWrapperWithData<Self, D, S>{
            BuilderWrapperWithData(self, data, PhantomData::default())
        }

        fn wrap(self) -> BuilderWrapper<Self, S>{
            BuilderWrapper(self, PhantomData::default())
        }
    }

    pub fn new_spec_builder(name: String, optional:bool)-> ProtoSpecBuilderData<BuildFromScratch>{
        ProtoSpecBuilderData::<BuildFromScratch>::new_with(name, optional)
    }

    impl <S> ProtoSpecBuilderData<S> where S:BuilderState {
        pub fn new_with_state(state: S) -> Self {
            ProtoSpecBuilderData {
                composite_spec: ListSpec { 
                    spec_meta_data: {
                        SpecMetaData::new("root".to_owned(), ValueType::None, true)
                    },
                    constituents: Vec::new() 
                },
                state,
            }
        }

        pub fn new() -> Self {
            ProtoSpecBuilderData::new_with_state(S::default())
        }        

        pub fn new_with(name: String, optional: bool) -> Self {
            let mut result = ProtoSpecBuilderData::new_with_state(S::default());
            result.composite_spec.spec_meta_data = SpecMetaData::new(name, ValueType::None, optional);
            result
        }

        pub fn new_from_scratch(name: String, optional: bool) -> ProtoSpecBuilderData<BuildFromScratch> {
            let mut result = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default());
            result.composite_spec.spec_meta_data = SpecMetaData::new(name, ValueType::None, optional);
            result
        }        

        /* pub fn add_spec(mut self, spec: Box<dyn Spec>) -> Self{
            //self.state.get_mut_shared_state_data().add_spec(spec);
            self.composite_spec.add_spec(spec);
            self
        } */


    }
    
    //Generators

    pub trait NumberSpecGenerator {
        fn get_u16_spec(&self, name: String, optional: bool) -> NumberU16Spec{
            NumberU16Spec(SpecMetaData::new(name, ValueType::UnSignedNumber16, optional))       
        }
        fn get_u32_spec(&self, name: String, optional: bool) -> NumberU32Spec{
            NumberU32Spec(SpecMetaData::new(name, ValueType::UnSignedNumber32, optional))       
        }
        fn get_u64_spec(&self, name: String, optional: bool) -> NumberU64Spec{
            NumberU64Spec(SpecMetaData::new(name, ValueType::UnSignedNumber64, optional))       
        }
        fn get_i16_spec(&self, name: String, optional: bool) -> NumberI16Spec{
            NumberI16Spec(SpecMetaData::new(name, ValueType::SignedNumber16, optional))       
        }
        fn get_i64_spec(&self, name: String, optional: bool) -> NumberI64Spec{
            NumberI64Spec(SpecMetaData::new(name, ValueType::SignedNumber64, optional))
        }
    }

    trait StringSpecGenerator{
        fn get_string_spec(&self, name: String, optional: bool) -> DelimitedStringSpec where  Self:Sized{
            DelimitedStringSpec { 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, optional), 
                until: Separator::EndOfStream 
            }
        }

        fn get_one_of_string(&self, name: String, optional: bool, options: Vec<String>) ->  OneOfSpec where Self:Sized{
            OneOfSpec{ 
                spec_meta_data: SpecMetaData::new(name, ValueType::String, 
                optional), until: Separator::EndOfStream,  
                values: options 
            }
        }

        fn get_exact_string(&self, name: String, input: String, optional: bool) -> ExactStringSpec where Self:Sized {
            ExactStringSpec::new(name, input, optional)
        }
    }

    trait KeySpecGenerator{
        fn get_key_spec(&self, name: String, optional: bool) -> Key{
            let mut spec= DelimitedStringSpec::default();
            spec.spec_meta_data = SpecMetaData::new(name.clone(), ValueType::String, optional);
            Key(Box::new(spec), SpecMetaData::new(format!("key-for-{}", name) , ValueType::None, optional))
        }
    }


    // Generator impls
    /* impl NumberSpecGenerator for ProtoSpecBuilderData<BuildFromScratch>{}
    impl NumberSpecGenerator for ProtoSpecBuilderData<BuildValue>{}
    impl NumberSpecGenerator for ProtoSpecBuilderData<BuildInlineValue>{}
    impl NumberSpecGenerator for ProtoSpecBuilderData<BuildKeyAvailable>{} */
    

    impl <S> StringSpecGenerator for ProtoSpecBuilderData<S> where S:BuilderState{}

    impl KeySpecGenerator for ProtoSpecBuilderData<BuildFromScratch>{}


        
    //Spec Builders

    /* impl  <IBS, OBS, S> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, S, IBS>> for ProtoSpecBuilderData<OBS>
    where 
        S: Spec + 'static,
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
    {
        fn from(mut value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, NumberI16Spec, IBS>) -> Self {
            let mut to_builder = ProtoSpecBuilderData::default();
            to_builder.set_spec(value.0.build());
            to_builder.add_spec(Box::new(value.1));
            to_builder
        }
    } */

    /* impl <S>  From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildValue>, S, BuildValue>> for ProtoSpecBuilderData<BuildFromScratch>
    where 
    S: Spec + 'static,
    {
        fn from(mut value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildValue>, NumberI16Spec, BuildValue>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let optional = from_state.key.1.optional;
            let spec = KeyValueSpec::new(
                from_state.key,
                ValueSpec(Box::new(value.1), from_state.value_spec_metadata), 
                SpecMetaData::new("key-value-spec".to_owned(), ValueType::None, optional)
            );
            from_builder.add_spec(Box::new(spec));
            let mut to_builder = ProtoSpecBuilderData::default();
            to_builder.set_spec(from_builder.build());
            to_builder

        }
    } */
    
    impl <IBS> NumberSpecGenerator for ProtoSpecBuilderData<IBS> where IBS: BuilderState + 'static{}

    /* impl <IBS>  NumberSpecBuilder <IBS, IBS, ProtoSpecBuilderData<IBS>> 
    for ProtoSpecBuilderData<IBS> where IBS:BuilderState + 'static{}
 */
    impl NumberSpecBuilder <BuildValue, BuildFromScratch, ProtoSpecBuilderData<BuildFromScratch>> 
    for ProtoSpecBuilderData<BuildValue>{}

    impl NumberSpecBuilder <BuildFromScratch, BuildKeyAvailable, ProtoSpecBuilderData<BuildKeyAvailable>> 
    for ProtoSpecBuilderData<BuildFromScratch>{}

    impl  NumberSpecBuilder<BuildInlineValue, BuildFromScratch, ProtoSpecBuilderData<BuildFromScratch>>
    for ProtoSpecBuilderData<BuildInlineValue>{}

    impl  NumberSpecBuilder<BuildKeyAvailable, BuildKeyAvailable, ProtoSpecBuilderData<BuildKeyAvailable>>
    for ProtoSpecBuilderData<BuildKeyAvailable>{}
    
    pub trait CustomSpecBuilder<IBS>: ProtoSpecBuilder<IBS>
    where IBS: BuilderState + 'static,
    {
        fn use_spec(mut self, spec: Box<dyn Spec>) -> Self{
            self.add_spec(spec);
            self
        }
    }

    impl CustomSpecBuilder<BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>{
    }

    pub trait NumberSpecBuilder <IBS,OBS, OB> :NumberSpecGenerator + ProtoSpecBuilder<IBS>
    where 
        Self: Sized + ProtoSpecBuilder<IBS> + 'static, 
        OB: ProtoSpecBuilder<OBS> + 'static,
        OBS: BuilderState + 'static, 
        IBS: BuilderState + 'static,        
        {
            
        fn expect_u16(self, name: String, optional: bool ) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU16Spec, IBS>> + 'static,            
        {        
            let mut spec = self.get_u16_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_u32(self, name: String, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU32Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_u32_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_u64(self, name: String, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberU64Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_u64_spec(name, optional);
            self.wrap_with_data(spec).into()
        }

        fn expect_i16(self, name: String, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberI16Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_i16_spec(name, optional);
            self.wrap_with_data(spec).into() 
        }

        fn expect_i64(self, name: String, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where 
        OBS: BuilderState +  'static,
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, NumberI64Spec, IBS>> + 'static,            
        {
            let mut spec = self.get_i64_spec(name, optional);
            self.wrap_with_data(spec).into()
        }
    }

    pub trait InlineValueBuilder <IBS, OBS> :StringSpecGenerator + ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
        OBS:BuilderState + 'static, 
    {

        fn inline_value_follows(self, key_name: String, optional: bool) ->  ProtoSpecBuilderData<OBS>//impl ProtoSpecBuilder<BuildDelimiter<DelimitedStringSpec, IBS>> 
        where                        
        ProtoSpecBuilderData<OBS>:ProtoSpecBuilder<BuildInlineValue> + From<BuilderWrapperWithData<Self, BuildInlineValue, IBS>>  + 'static,            

        {
            self.wrap_with_data(BuildInlineValue{
                key_name:key_name.clone(),
                value_spec_metadata:SpecMetaData { name: format!("value-for-{}", key_name), value_type: ValueType::None, optional: optional }
            }).into()
        }
    }
    
    impl From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildFromScratch>, BuildInlineValue, BuildFromScratch>> for ProtoSpecBuilderData<BuildInlineValue>{
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildFromScratch>, BuildInlineValue, BuildFromScratch>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let inline_value_data  = value.1;
            let mut to_builder = ProtoSpecBuilderData::default();
            to_builder.set_state(inline_value_data);
            to_builder.set_spec(from_builder.composite_spec);
            to_builder
        }
    }

 /*    impl From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildKeyAvailable>, SpecMetaData, BuildKeyAvailable>> for ProtoSpecBuilderData<BuildValue>{
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildKeyAvailable>, SpecMetaData, BuildKeyAvailable>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let value_spec_metadata = value.1;
            let mut result_builder = ProtoSpecBuilderData::default();
            let result_state = BuildValue{
                key: from_state.key,
                value_spec_metadata: value_spec_metadata,   
            };
            result_builder.set_spec(from_builder.composite_spec);
            result_builder.set_state(result_state);
            result_builder
        }
    } */

    pub trait ValueBuilder <IBS> : ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
        
    {

        fn value_follows(self, name: String, optional: bool) ->  ProtoSpecBuilderData<BuildValue>
        where ProtoSpecBuilderData<BuildValue>: From<BuilderWrapperWithData<Self, SpecMetaData, IBS>>
        

        {
            self.wrap_with_data(SpecMetaData::new(name, ValueType::None, optional)).into()
        }
    }

    impl ValueBuilder<BuildKeyAvailable> for ProtoSpecBuilderData<BuildKeyAvailable>{}

    impl  InlineValueBuilder<BuildFromScratch, BuildInlineValue> for ProtoSpecBuilderData<BuildFromScratch>{}

    

    struct Repeat(ListSpec, RepeatTimes);

    enum RepeatTimes{
        RepeatN(u8),
        RepeatMany,
    }

    struct RepeatSpec{
        spec_metadata: SpecMetaData,
        composite_spec: ListSpec,
        repeat_times: RepeatTimes,
    }

    impl Spec for RepeatSpec{
        fn get_meta_data(&self) -> &SpecMetaData {
            &self.spec_metadata
        }
    }

    #[async_trait]
    impl SpecSerialize for RepeatSpec{

        async fn serialize(
            &self,
            info_provider: &(dyn InfoProvider + Send + Sync),
            writer: &mut (dyn SpecWrite),            
        ) -> Result<(), ParserError>
        {
            //let name = self.key.get_meta_data().get_name();
            todo!("implement");
            //self.key.serialize(info_provider, writer).await?;
            /* let value = info_provider.get_info(name);
            write_data(name.to_owned(), value, self.spec_metadata.optional, writer).await?; */
            
        }
        
    }

    #[async_trait]
    impl SpecDeserialize for RepeatSpec{
        async fn deserialize (
            &self,
            info_provider: &mut ( dyn InfoProvider + Send + Sync ),
            reader: &mut (dyn SpecRead),
        ) -> Result<Value, ParserError>{
            match self.repeat_times{
                RepeatTimes::RepeatN(times) => {
                    for i in 0..times {
                        self.composite_spec.undoable_parse(info_provider, reader).await;
                    }
                    return Ok(Value::None)
                },
                RepeatTimes::RepeatMany => {
                    loop{
                        let result = self.composite_spec.undoable_parse(info_provider, reader).await;
                        if result.is_err() {
                            //check error type
                            return Ok(Value::None);
                            todo!("check error type before returning Ok")
                        }
                    }
                }
            }
        }
    }
    pub trait CompositeBuilder<IBS, OBS>: ProtoSpecBuilder<IBS>
    where 
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
        Self: Sized + 'static
    {

        fn expect_composite(self, spec: ListSpec) -> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ListSpec, IBS>>,
        {
            //self.add_spec(Box::new(spec));
            self.wrap_with_data(spec).into()
        }
    }

    impl CompositeBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch> {}

    /* impl <IBS, OBS> From<BuilderWrapperWithData<Self, ListSpec, IBS>> for ProtoSpecBuilderData<OBS>
    where 
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
    {
        fn from(mut value: BuilderWrapperWithData<Self, ListSpec, IBS>) -> Self {
            let mut result = ProtoSpecBuilderData::default();
            result.set_state(OBS::default());
            result.set_spec(value.0.composite_spec);
            result.add_spec(Box::new(value.1));
            result
        }
    } */



    pub trait RepeatBuilder<IBS, OBS>: ProtoSpecBuilder<IBS>
    where 
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
        Self: Sized + 'static
    {

        fn repeat_many(self, name: Option<String>, optional: bool, spec: ListSpec) -> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, RepeatSpec, IBS>>,
        {
            let name = name.unwrap_or("repeat_spec".to_owned());
            let repeat_spec = RepeatSpec{
                spec_metadata: SpecMetaData::new(name, ValueType::None, optional),
                composite_spec: spec,
                repeat_times: RepeatTimes::RepeatMany,
            };
            self.wrap_with_data(repeat_spec).into()
        }

        fn repeat_n_times(self, name: String, optional: bool, number_of_times: u8, spec: ListSpec) -> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, RepeatSpec, IBS>>,
        {
            let repeat_spec = RepeatSpec{
                spec_metadata: SpecMetaData::new(name, ValueType::None, optional),
                composite_spec: spec,
                repeat_times: RepeatTimes::RepeatN(number_of_times),
            };
            self.wrap_with_data(repeat_spec).into()
        }
    }

    impl RepeatBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>{}

    pub trait DelimitedStringSpecBuilder <IBS> :StringSpecGenerator + ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
    //    OBS:BuilderState + 'static, 
    {

        fn expect_string(self, name: Option<String>, optional: bool) ->  ProtoSpecBuilderData<BuildDelimiter<DelimitedStringSpec, IBS>>  //impl ProtoSpecBuilder<BuildDelimiter<OneOfSpec, IBS>>
        where                        
        ProtoSpecBuilderData<BuildDelimiter<DelimitedStringSpec, IBS>>:From<BuilderWrapperWithData<Self, DelimitedStringSpec, IBS>> + 'static,
        
        {            
            let name = name.unwrap_or("expect_string".to_string());
            let mut spec = self.get_string_spec(name, optional);
            self.wrap_with_data(spec).into()            
        }

        fn expect_one_of_string(self, name: Option<String>, optional: bool, options: Vec<String>) ->  ProtoSpecBuilderData<BuildDelimiter<OneOfSpec, IBS>>  //impl ProtoSpecBuilder<BuildDelimiter<OneOfSpec, IBS>>
        where
        ProtoSpecBuilderData<BuildDelimiter<OneOfSpec, IBS>>:From<BuilderWrapperWithData<Self, OneOfSpec, IBS>> + 'static
        {
            let name = name.unwrap_or("expect_one_of_string".to_string());
            let mut one_of_spec = OneOfSpec::new(name, optional, options);
            self.wrap_with_data(one_of_spec).into()            
        }

    }

    pub trait StringSpecBuilder <IBS, OBS> :StringSpecGenerator + ProtoSpecBuilder<IBS>  
    where 
        Self: Sized + 'static,
        IBS: BuilderState + 'static,
        OBS:BuilderState + 'static, 
    {


        fn expect_exact_string(self, name: Option<String>, input: String, optional: bool) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            //OB: ProtoSpecBuilder<OBS> + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,            
        {
            let name = name.unwrap_or("expect_exact_string".to_string());
            let exact_string = ExactStringSpec::new(name, input, optional);
            self.wrap_with_data(exact_string).into()            
            //From::from(self.wrap_with_data(exact_string))
        }

        fn expect_newline(self) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            //OB: ProtoSpecBuilder<OBS> + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,{
                self.expect_exact_string(None, "\r\n".to_string(), false)
            }

        fn expect_space(self,) -> ProtoSpecBuilderData<OBS> 
        where
            Self: Sized + 'static,
            //OB: ProtoSpecBuilder<OBS> + 'static,
            OBS: BuilderState +  'static,
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, ExactStringSpec, IBS>> + 'static,{
                self.expect_exact_string(None, " ".to_string(), false)
            }
    }

   

    pub trait KeySpecBuilder<IBS>: KeySpecGenerator + ProtoSpecBuilder<IBS>
    where 
        Self:Sized,
        IBS: BuildGenericString,
    {
        fn key_follows(self, name: String, optional: bool) -> ProtoSpecBuilderData<BuildKey>
        /* where 
            OB: ProtoSpecBuilder<BuildKey> */
        {
            
            let mut result= ProtoSpecBuilderData::default();
            result.set_spec(self.build());
            result.set_state(BuildKey { key_spec_metadata: SpecMetaData::new(name, ValueType::None, optional) });
            result
        }
    }

    fn generate_key_name() -> String{
        "key-1".to_string()
    }

    impl KeySpecBuilder<BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch> {}


    impl  StringSpecBuilder<BuildKey, BuildKeyAvailable> for ProtoSpecBuilderData<BuildKey>     
    {       
    }

    impl  StringSpecBuilder<BuildInlineValue, BuildFromScratch> for ProtoSpecBuilderData<BuildInlineValue>     
    {       
    }

    /* impl  StringSpecBuilder<BuildKeyAvailable, BuildValue> for ProtoSpecBuilderData<BuildKeyAvailable>     
    {       
    } */

    impl  StringSpecBuilder<BuildKeyAvailable, BuildKeyAvailable> for ProtoSpecBuilderData<BuildKeyAvailable>     
    {       
    }

    impl  StringSpecBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>     
    {       
    }

    

    /* impl  StringSpecBuilder<BuildValue, BuildValue> for ProtoSpecBuilderData<BuildValue>     
    {       
    }  */

    impl  StringSpecBuilder<BuildValue, BuildFromScratch> for ProtoSpecBuilderData<BuildValue>     
    {       
    }

    
    /* impl <IBS> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, String, IBS>> for ProtoSpecBuilderData<IBS>
    where 
          IBS: BuilderState + 'static,
          
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, String, IBS>) -> Self {
            let mut from_builder = value.0;
            //from_builder.add_spec(Box::new(delimited_spec));
            from_builder
        }
    } */

    /* where                        
        ProtoSpecBuilderData<BuildDelimiter<DelimitedStringSpec, IBS>>:From<BuilderWrapperWithData<Self, DelimitedStringSpec, IBS>> + 'static, */

    impl  DelimitedStringSpecBuilder<BuildInlineValue> for ProtoSpecBuilderData<BuildInlineValue>     
    
    {       
    }

    impl DelimitedStringSpecBuilder<BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>     
    
    {       
    }

    impl DelimitedStringSpecBuilder<BuildKey> for ProtoSpecBuilderData<BuildKey>     
    {       
    }

    impl DelimitedStringSpecBuilder<BuildValue> for ProtoSpecBuilderData<BuildValue>     
    {       
    }

    impl DelimitedStringSpecBuilder<BuildKeyAvailable> for ProtoSpecBuilderData<BuildKeyAvailable>     
    {       
    }

    /* impl <D> StringSpecBuilder<BuildKeyAvailable, BuildDelimiter<D, BuildKeyAvailable>> for ProtoSpecBuilderData<BuildKeyAvailable>     
    where D: DelimitedSpec + 'static,
    {       
    } */

    /* impl <D> StringSpecBuilder<BuildDelimiter<D, BuildInlineValue>, BuildFromScratch> for ProtoSpecBuilderData<BuildDelimiter<D, BuildInlineValue>>     
    where D: DelimitedSpec + 'static,
    {       
    } */

    /* impl <D> StringSpecBuilder<BuildDelimiter<D, BuildValue>, BuildFromScratch> for ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>     
    where D: DelimitedSpec + 'static,
    {       
    } */
    

    /* impl <D> StringSpecBuilder<BuildDelimiter<D, BuildValue>, BuildValue> for ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>
    where D: DelimitedSpec + 'static,
    {       
    } */

    /* impl <D> StringSpecBuilder<BuildDelimiter<D, BuildKeyAvailable>, BuildKeyAvailable> for ProtoSpecBuilderData<BuildDelimiter<D, BuildKeyAvailable>>
    where D: DelimitedSpec + 'static,
    {       
    } */

    

    

    fn test(){
        //let t: ProtoSpecBuilderData<BuildKeyAvailable> = ProtoSpecBuilderData::new(BuildFromScratch::default()).expect_u16("rar".to_owned(), true);
         let t= ProtoSpecBuilderData::new_with_state(BuildFromScratch::default());
            let t = t.key_follows("keyname".to_string(), false);
            let t = t.expect_string(None, true)
            .delimited_by_newline()
            //.expect_exact_string("name".to_owned(), "input".to_owned(), false)
            
            ;
        let t = t.expect_string(None,false);
        let t: ProtoSpecBuilderData<BuildKeyAvailable> = t.delimited_by_space();
        let t: ProtoSpecBuilderData<BuildKeyAvailable> = t.expect_exact_string(None, "test".to_string(), false);
        
        //t.expect_delimiter("dem".to_owned(), "delin".to_owned(), false);
            /* let t1= t.expect_exact_string("test".to_owned(), "delimiter".to_owned(), true);
            //let t1 = t1.expect_string("newstr".to_owned(), false);
            ``
            let  t1 = t1.expect_i16("name".to_owned(), false);
            let t1 = t1.expect_i16("tet".to_owned(), true); */
            
            //let x= t.expect_exact_string("fasf".to_owned(), "test".to_owned(),  false);
            let x = t.expect_one_of_string(None,  false, vec!());
            let x = x.delimited_by_space(); 
            let x = x.value_follows("test".to_string(), true)  ;
            let x = x.expect_string(None,  true);
            let x = x.delimited_by("\r\n".to_owned());
            let x = x.expect_exact_string(None, "test".to_string(), true);



            let x = x.repeat_many(Some("repeat".to_string()), true, ListSpec::new("test".to_owned(), ValueType::None, false));
                
                
            
            //x.expect_string("name".to_owned(), false);
                
                
    }

    

    struct BuilderWrapperWithData<B,D, BS>(B, D , PhantomData<BS> ) 
    where
        B:ProtoSpecBuilder<BS> + 'static, 
        BS:BuilderState + 'static;
    struct BuilderWrapper<B,BS>(B , PhantomData<BS> ) where B:ProtoSpecBuilder<BS> + 'static, BS:BuilderState + 'static;

     impl <D, IBS> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>> for ProtoSpecBuilderData<IBS> 
     where 
         D:Spec + 'static,
         IBS:BuilderState + 'static,
        
     {
         fn from(mut value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>) -> Self 
         {
             let from_builder = &mut value.0;             
             from_builder.add_spec(Box::new(value.1));
             value.0
         }
     }

    /*  impl <IBS, D> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>> for ProtoSpecBuilderData<BuildDelimiter<D, IBS>>
    where D: DelimitedSpec
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>) -> Self {
            todo!()
        }
    } */
    

     impl <D, IBS> From<BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>> for ProtoSpecBuilderData<BuildDelimiter<D, IBS>> 
     where 
         D:DelimitedSpec + 'static,
         IBS:BuilderState + 'static,
        
     {
         fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<IBS>, D, IBS>) -> Self 
         {
             let mut from_builder =  value.0;             
             
             
             let mut to_builder = ProtoSpecBuilderData::default();             
             let to_state = BuildDelimiter{
                delimiter_spec: value.1,
                parent_builder_state: from_builder.replace_current_state_with_default(),
             };
             to_builder.set_spec(from_builder.build());
             to_builder.set_state(to_state);             
             to_builder
         }
     }

     

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKey>>, String , BuildDelimiter<D, BuildKey>>> for ProtoSpecBuilderData<BuildKeyAvailable> 
    where 
        D:DelimitedSpec + StringSpec + 'static,        
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKey>>, String, BuildDelimiter<D, BuildKey>>) -> Self 
        {
            let mut from_builder = value.0;            
            let from_state = from_builder.replace_current_state_with_default();
            let mut spec = from_state.delimiter_spec;
            spec.set_delimiter(Separator::Delimiter(value.1));
            let mut result = ProtoSpecBuilderData::default();
            let key = Key(Box::new(spec), from_state.parent_builder_state.key_spec_metadata);
            result.set_state(BuildKeyAvailable{
                key
            });
            result.set_spec(from_builder.build());
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildFromScratch>>, String , BuildDelimiter<D, BuildFromScratch>>> for ProtoSpecBuilderData<BuildFromScratch> 
    where 
        D:DelimitedSpec + StringSpec + 'static,        
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildFromScratch>>, String, BuildDelimiter<D, BuildFromScratch>>) -> Self 
        {
            let mut from_builder = value.0;            
            let from_state = from_builder.replace_current_state_with_default();
            let mut spec = from_state.delimiter_spec;
            spec.set_delimiter(Separator::Delimiter(value.1));
            let mut result = ProtoSpecBuilderData::default();
            let new_state = from_state.parent_builder_state;
            result.set_state(new_state);
            result.set_spec(from_builder.build());
            result.add_spec(Box::new(spec));
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKeyAvailable>>, String , BuildDelimiter<D, BuildKeyAvailable>>> for ProtoSpecBuilderData<BuildKeyAvailable> 
    where 
        D:DelimitedSpec + StringSpec + 'static,        
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildKeyAvailable>>, String, BuildDelimiter<D, BuildKeyAvailable>>) -> Self 
        {
            let mut from_builder = value.0;            
            let from_state = from_builder.replace_current_state_with_default();
            let mut spec = from_state.delimiter_spec;
            spec.set_delimiter(Separator::Delimiter(value.1));
            let mut result = ProtoSpecBuilderData::default();
            let new_state = from_state.parent_builder_state;
            result.set_state(new_state);
            result.set_spec(from_builder.build());
            result.add_spec(Box::new(spec));
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildKey>, D, BuildKey>> for ProtoSpecBuilderData<BuildKeyAvailable>
     where D:StringSpec + 'static{
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildKey>, D, BuildKey>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let key = Key(Box::new(value.1), from_state.key_spec_metadata);
            result.set_state(BuildKeyAvailable { key: key });        
            //result.set_state(output_state);
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl  From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildKeyAvailable>, SpecMetaData, BuildKeyAvailable>> for ProtoSpecBuilderData<BuildValue>
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildKeyAvailable>, SpecMetaData, BuildKeyAvailable>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            
            result.set_state(BuildValue { key: from_state.key, value_spec_metadata: value.1 });        
            //result.set_state(output_state);
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildValue>, D, BuildValue>> for ProtoSpecBuilderData<BuildFromScratch>
    where D: Spec + 'static
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildValue>, D, BuildValue>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let optional = from_state.key.1.optional;
            let key_value = KeyValueSpec::new(
                from_state.key,
                ValueSpec(Box::new(value.1), from_state.value_spec_metadata),
                SpecMetaData::new("key-value-spec".to_owned(), ValueType::None, optional),
            );
            from_builder.add_spec(Box::new(key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }
             
    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>, String, BuildDelimiter<D, BuildValue>>> for ProtoSpecBuilderData<BuildFromScratch>
    where D: DelimitedSpec + 'static,
           // IBS: BuilderState + 'static,
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>, String, BuildDelimiter<D, BuildValue>>) -> Self {
            let mut from_builder = value.0;
            let mut from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let optional = from_state.parent_builder_state.key.1.optional;
            from_state.delimiter_spec.set_delimiter(Separator::Delimiter(value.1));
            let key_value = KeyValueSpec::new(
                from_state.parent_builder_state.key,
                ValueSpec(Box::new(from_state.delimiter_spec), from_state.parent_builder_state.value_spec_metadata),
                SpecMetaData::new("key-value-spec".to_owned(), ValueType::None, optional),
            );
            from_builder.add_spec(Box::new(key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }

    impl <D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildInlineValue>>, String, BuildDelimiter<D, BuildInlineValue>>> for ProtoSpecBuilderData<BuildFromScratch>
    where D: DelimitedSpec + 'static,
           // IBS: BuilderState + 'static,
     {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, BuildInlineValue>>, String, BuildDelimiter<D, BuildInlineValue>>) -> Self {
            let mut from_builder = value.0;
            let mut from_state = from_builder.replace_current_state_with_default();
            let mut result = ProtoSpecBuilderData::default();
            let optional = from_state.parent_builder_state.value_spec_metadata.optional;
            from_state.delimiter_spec.set_delimiter(Separator::Delimiter(value.1));
            let inline_key_value = InlineKeyWithValue(Box::new(from_state.delimiter_spec), from_state.         parent_builder_state.key_name, from_state.parent_builder_state.value_spec_metadata);
            from_builder.add_spec(Box::new(inline_key_value));
            result.set_state(BuildFromScratch{});
            result.set_spec(from_builder.build());    
            result
        }
    }

    //Generators
    impl <OBS> DelimiterGenerator for ProtoSpecBuilderData<OBS> 
    where 
        OBS: BuilderState
    {}


    trait DelimiterGenerator{
        fn get_newline(&self) -> Separator{
            Separator::Delimiter("\r\n".to_owned())
        }
        fn get_delimiter(&self, delimiter:String) -> Separator{
            Separator::Delimiter(delimiter)
        }
        fn get_space(&self) -> Separator {
            Separator::Delimiter(" ".to_owned())
        }
    }

    pub trait DelimiterBuilder<D,IBS,  OBS>: ProtoSpecBuilder<BuildDelimiter<D, IBS>> + DelimiterGenerator
    where 
        D: DelimitedSpec + 'static,
        IBS: BuilderState + 'static,
        OBS: BuilderState + 'static,
        Self: Sized + 'static,
    {
        fn delimited_by_newline(self)-> ProtoSpecBuilderData<OBS>
        where 
            
            ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
            
        {
            self.create_delimiter("\r\n".to_owned())
        }

        

        fn delimited_by_space(self,)-> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
        {
            self.create_delimiter( " ".to_string())
        }

        fn delimited_by(self, delimiter: String)-> ProtoSpecBuilderData<OBS>
        where ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
        {
            self.create_delimiter( delimiter)
        }

        fn create_delimiter(self, delimiter: String,) -> ProtoSpecBuilderData<OBS>
        where 
        ProtoSpecBuilderData<OBS>: From<BuilderWrapperWithData<Self, String, BuildDelimiter<D, IBS>>>,
        {
            //let delimiter_spec = Separator::Delimiter(delimiter);
            let r = self.wrap_with_data(delimiter);
            r.into()
        }
    }

    /* impl DelimiterBuilder<BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>
    {} */

    /* impl <PBS, D> DelimiterBuilder<BuildDelimiter<D, PBS>, PBS> for ProtoSpecBuilderData<BuildDelimiter<D, PBS>>
    where D: DelimitedSpec + 'static,
          PBS: BuilderState + 'static
    {} */

   impl <D, IBS, OBS> DelimiterBuilder<D, IBS, OBS> for ProtoSpecBuilderData<BuildDelimiter<D, IBS>>
   where D: DelimitedSpec + 'static,
         IBS: BuilderState + 'static,
         OBS: BuilderState + 'static,
        
   {}

    /* impl <D> DelimiterBuilder<D, BuildDelimiter<D, BuildKey>, BuildKeyAvailable> for ProtoSpecBuilderData<BuildDelimiter<D, BuildKey>>
    where D: DelimitedSpec + 'static,
    {} */

    /* impl <D> DelimiterBuilder<D, BuildFromScratch, BuildFromScratch> for ProtoSpecBuilderData<BuildFromScratch>
    where D:DelimitedSpec + 'static
    {} */

    /* impl <D> DelimiterBuilder<D, BuildDelimiter<D, BuildValue>, BuildFromScratch> for ProtoSpecBuilderData<BuildDelimiter<D, BuildValue>>
    where D: DelimitedSpec + 'static,
    {}
 */
    /* impl <D> DelimiterBuilder<D, BuildDelimiter<D, BuildKeyAvailable>, BuildKeyAvailable> for ProtoSpecBuilderData<BuildDelimiter<D, BuildKeyAvailable>>
    where D: DelimitedSpec + 'static,
    {}

    impl <D> DelimiterBuilder<D, BuildDelimiter<D, BuildFromScratch>, BuildFromScratch> for ProtoSpecBuilderData<BuildDelimiter<D, BuildFromScratch>>
    where D: DelimitedSpec + 'static,
    {}

    impl <D> DelimiterBuilder<D, BuildDelimiter<D, BuildInlineValue>, BuildFromScratch> for ProtoSpecBuilderData<BuildDelimiter<D, BuildInlineValue>>
    where D: DelimitedSpec + 'static,
    {} */



    /* impl <IBS, D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, IBS>>, String, BuildDelimiter<D, IBS>>> for ProtoSpecBuilderData<IBS>
    where 
          IBS: BuilderState + 'static,
          D: DelimitedSpec + 'static
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, IBS>>, String, BuildDelimiter<D, IBS>>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut to_builder = Self::default();
            let mut delimited_spec = from_state.delimiter_spec;
            delimited_spec.set_delimiter(Separator::Delimiter(value.1));
            to_builder.set_spec(from_builder.composite_spec);
            to_builder.add_spec(Box::new(delimited_spec));
            to_builder.set_state(from_state.parent_builder_state);    
            to_builder
        }
    } */

    /* impl <IBS, D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, IBS>>, String, BuildDelimiter<D, IBS>>> for ProtoSpecBuilderData<IBS>
    where 
          IBS: BuilderState + 'static,
          //OBS: BuilderState + 'static,
          D: DelimitedSpec + 'static
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, IBS>>, String, BuildDelimiter<D, IBS>>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut to_builder = Self::default();
            let mut delimited_spec = from_state.delimiter_spec;
            delimited_spec.set_delimiter(Separator::Delimiter(value.1));
            to_builder.set_spec(from_builder.composite_spec);
            to_builder.add_spec(Box::new(delimited_spec));
            to_builder.set_state(from_state.parent_builder_state);    
            to_builder
        }
    } */ 

    /* impl <IBS, OBS, D> From<BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, IBS>>, Separator, BuildDelimiter<D, IBS>>> for ProtoSpecBuilderData<OBS>
    where 
          IBS: BuilderState + 'static,
          OBS: BuilderState + 'static,
          D: DelimitedSpec + 'static
    {
        fn from(value: BuilderWrapperWithData<ProtoSpecBuilderData<BuildDelimiter<D, IBS>>, Separator, BuildDelimiter<D, IBS>>) -> Self {
            let mut from_builder = value.0;
            let from_state = from_builder.replace_current_state_with_default();
            let mut to_builder = Self::default();
            let mut delimited_spec = from_state.delimiter_spec;
            delimited_spec.set_delimiter(value.1);
            to_builder.set_spec(from_builder.composite_spec);
            to_builder.add_spec(Box::new(delimited_spec));
            to_builder.set_state(from_state.parent_builder_state);    
            to_builder
        }
    } */

    pub(crate) mod protocol_reader;
    mod protocol_writer;
}

pub mod http;
mod utils;

#[cfg(test)]
mod tests {
    use crate::core::{
        BuildFromScratch, DelimiterBuilder, InlineValueBuilder, ProtoSpecBuilderData, StringSpecBuilder, DelimitedStringSpecBuilder
    };

    

    fn test_string_placeholder(){
        let mut spec_builder = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default());
        let spec = spec_builder.expect_string(None, false);
        let spec = spec.delimited_by_space();
                       
    }

    #[test]
    fn test_protocol_spec_builder() {
        let mut spec_builder = ProtoSpecBuilderData::new_with_state(BuildFromScratch::default());


        let spec_builder = spec_builder.inline_value_follows("key-1".to_owned(), false);
    }
}

#[cfg(test)]
mod test_utils {
    use std::collections::HashMap;

    use crate::core::{InfoProvider, Value};

    pub fn assert_result_has_string(
        result: Result<Option<Vec<u8>>, crate::core::ParserError>,
        data: String,
    ) {
        match result {
            Ok(Some(result_data)) => {
                assert!(data == String::from_utf8(result_data).unwrap());
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[derive(Default)]
    pub struct TestRequestInfo(HashMap<String, Value>, HashMap<String, Value>, HashMap<String, HashMap<String, Value>>);

    impl TestRequestInfo {
        pub fn new() -> Self {
            TestRequestInfo(HashMap::new(), HashMap::new(), HashMap::new())
        }
    }

    impl InfoProvider for TestRequestInfo {
        fn add_info(&mut self, key: String, value: Value) {
            self.0.insert(key, value);
        }

        fn get_info(&self, key: &String) -> Option<&crate::core::Value> {
            if let Some(value) = self.0.get(key) {
                Some(value)
            }else if let Some(value) = self.1.get(key){
                Some(value)
            }
            else {
                None
            }
        }

        fn get_keys_by_group_name(&self, _name: String) -> Option<Vec<&String>> {
            let mut keys = Vec::new();
            for (key, _value) in &self.0 {
                keys.push(key);
            }
            if keys.is_empty() {
                None
            } else {
                Some(keys)
            }
        }

        fn get_info_mut(&mut self, key: &String) -> Option<&mut Value> {
            if let Some(value) = self.0.get_mut(key) {
                Some(value)
            }else if let Some(value) = self.1.get_mut(key){
                Some(value)
            }
            else {
                None
            }
        }
        
        fn has_all_data(&self) -> bool {
            todo!()
        }
        
        /* fn add_transient_info(&mut self, key: String, value: Value) {
            self.1.insert(key, value);
        } */
    }
}
